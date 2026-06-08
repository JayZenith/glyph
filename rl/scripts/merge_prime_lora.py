#!/usr/bin/env python3
"""Merge a PRIME-RL filesystem-broadcast LoRA adapter into a HF base model.

PRIME-RL broadcast adapters use raw module paths such as
``model.layers.0.self_attn.q_proj.lora_A.weight``.  Those files are the
artifact that vLLM trains/serves during LoRA RLVR, but they are not always
loadable as a normal PEFT adapter without key rewriting.  This script applies
the adapter deltas directly to the base model and saves a plain HF checkpoint
that can be evaluated with ``sft.eval_formal``.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

import torch
from safetensors.torch import load_file
from transformers import AutoModelForCausalLM, AutoTokenizer


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base-model", required=True, help="Base HF model or local path.")
    parser.add_argument(
        "--adapter-dir",
        required=True,
        help="PRIME-RL broadcast adapter directory containing adapter_config.json and adapter_model.safetensors.",
    )
    parser.add_argument("--output", required=True, help="Directory for the merged HF checkpoint.")
    parser.add_argument("--dtype", default="bfloat16", choices=("float16", "bfloat16", "float32"))
    parser.add_argument("--trust-remote-code", action="store_true")
    return parser.parse_args()


def dtype_from_name(name: str) -> torch.dtype:
    return {
        "float16": torch.float16,
        "bfloat16": torch.bfloat16,
        "float32": torch.float32,
    }[name]


def load_adapter_config(adapter_dir: Path) -> dict[str, Any]:
    path = adapter_dir / "adapter_config.json"
    if not path.exists():
        raise FileNotFoundError(f"missing adapter config: {path}")
    return json.loads(path.read_text())


def merge_adapter(model: torch.nn.Module, adapter_dir: Path, alpha: float, rank: int) -> tuple[int, int]:
    weights_path = adapter_dir / "adapter_model.safetensors"
    if not weights_path.exists():
        raise FileNotFoundError(f"missing adapter weights: {weights_path}")

    state = load_file(str(weights_path), device="cpu")
    lora_a: dict[str, torch.Tensor] = {}
    lora_b: dict[str, torch.Tensor] = {}
    for key, value in state.items():
        if key.endswith(".lora_A.weight"):
            lora_a[key[: -len(".lora_A.weight")]] = value
        elif key.endswith(".lora_B.weight"):
            lora_b[key[: -len(".lora_B.weight")]] = value

    if not lora_a:
        raise ValueError(f"no LoRA A matrices found in {weights_path}")

    applied = 0
    missing = 0
    scale = float(alpha) / float(rank)
    with torch.no_grad():
        for prefix, a_tensor in sorted(lora_a.items()):
            b_tensor = lora_b.get(prefix)
            if b_tensor is None:
                missing += 1
                continue
            try:
                module = model.get_submodule(prefix)
            except AttributeError:
                missing += 1
                continue
            if not hasattr(module, "weight"):
                missing += 1
                continue

            weight = module.weight
            delta = torch.matmul(b_tensor.float(), a_tensor.float()) * scale
            if tuple(delta.shape) != tuple(weight.shape):
                raise ValueError(f"shape mismatch for {prefix}: delta={tuple(delta.shape)} weight={tuple(weight.shape)}")
            weight.add_(delta.to(device=weight.device, dtype=weight.dtype))
            applied += 1

    return applied, missing


def main() -> int:
    args = parse_args()
    adapter_dir = Path(args.adapter_dir)
    output = Path(args.output)
    cfg = load_adapter_config(adapter_dir)
    rank = int(cfg["r"])
    alpha = float(cfg["lora_alpha"])

    tokenizer = AutoTokenizer.from_pretrained(args.base_model, trust_remote_code=args.trust_remote_code)
    model = AutoModelForCausalLM.from_pretrained(
        args.base_model,
        torch_dtype=dtype_from_name(args.dtype),
        device_map="cpu",
        trust_remote_code=args.trust_remote_code,
    )
    applied, missing = merge_adapter(model, adapter_dir, alpha=alpha, rank=rank)
    if applied == 0 or missing:
        raise RuntimeError(f"unsafe merge result: applied={applied}, missing={missing}")

    output.mkdir(parents=True, exist_ok=True)
    model.save_pretrained(output, safe_serialization=True, max_shard_size="5GB")
    tokenizer.save_pretrained(output)
    (output / "prime_lora_merge.json").write_text(
        json.dumps(
            {
                "base_model": args.base_model,
                "adapter_dir": str(adapter_dir),
                "rank": rank,
                "alpha": alpha,
                "applied_deltas": applied,
                "missing_deltas": missing,
            },
            indent=2,
        )
        + "\n"
    )
    print(f"Merged {applied} LoRA deltas into {output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
