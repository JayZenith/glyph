#!/usr/bin/env python3
"""
CPU-side audit for continuing PRIME-RL from a PEFT adapter on top of a frozen base.

Checks:
- adapter config resolves from local path or HF repo id
- base model + adapter load correctly with PEFT
- only LoRA tensors + modules_to_save are trainable
- modules_to_save includes lm_head
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

import torch
from huggingface_hub import snapshot_download
from peft import PeftConfig, PeftModel
from transformers import AutoModelForCausalLM


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--adapter",
        required=True,
        help="Local adapter dir or HF repo id, for example JayZenith/glyph-sft-v1-adapter",
    )
    parser.add_argument("--base-model", help="Optional base-model override.")
    parser.add_argument("--trust-remote-code", action="store_true")
    parser.add_argument("--output", type=Path, help="Optional JSON report path.")
    return parser.parse_args()


def resolve_adapter_dir(adapter: str) -> Path:
    path = Path(adapter)
    if path.exists():
        return path.resolve()
    return Path(snapshot_download(repo_id=adapter, repo_type="model"))


def main() -> int:
    args = parse_args()
    adapter_dir = resolve_adapter_dir(args.adapter)
    peft_cfg = PeftConfig.from_pretrained(str(adapter_dir))
    base_model_name = args.base_model or peft_cfg.base_model_name_or_path

    base = AutoModelForCausalLM.from_pretrained(
        base_model_name,
        torch_dtype=torch.bfloat16,
        device_map={"": "cpu"},
        trust_remote_code=args.trust_remote_code,
    )
    model = PeftModel.from_pretrained(base, str(adapter_dir), is_trainable=True)

    trainable = []
    frozen = 0
    for name, param in model.named_parameters():
        if param.requires_grad:
            trainable.append((name, param.numel()))
        else:
            frozen += param.numel()

    trainable_names = [name for name, _ in trainable]
    modules_to_save = list(peft_cfg.modules_to_save or [])
    allowed_non_lora_markers = tuple(
        marker
        for name in modules_to_save
        for marker in (
            f"{name}.",
            f"{name}.modules_to_save.default.",
        )
    )
    unexpected_trainable = [
        name
        for name in trainable_names
        if "lora_" not in name and not any(marker in name for marker in allowed_non_lora_markers)
    ]

    lora_params = sum(n for name, n in trainable if "lora_" in name)
    non_lora_trainable = [(name, n) for name, n in trainable if "lora_" not in name]
    non_lora_trainable_params = sum(n for _, n in non_lora_trainable)
    total_params = sum(p.numel() for p in model.parameters())

    report = {
        "adapter_source": args.adapter,
        "adapter_dir": str(adapter_dir),
        "base_model": base_model_name,
        "modules_to_save": modules_to_save,
        "expected_lm_head_trainable": "lm_head" in modules_to_save,
        "total_params": total_params,
        "frozen_params": frozen,
        "trainable_params": lora_params + non_lora_trainable_params,
        "trainable_fraction": (lora_params + non_lora_trainable_params) / total_params,
        "lora_trainable_params": lora_params,
        "non_lora_trainable_params": non_lora_trainable_params,
        "non_lora_trainable_tensors": [name for name, _ in non_lora_trainable],
        "unexpected_trainable_tensors": unexpected_trainable,
        "checks": {
            "loads_successfully": True,
            "modules_to_save_contains_lm_head": "lm_head" in modules_to_save,
            "only_lora_and_modules_to_save_trainable": not unexpected_trainable,
        },
    }

    text = json.dumps(report, indent=2)
    print(text)
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(text + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
