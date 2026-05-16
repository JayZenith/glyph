#!/usr/bin/env python3
"""
Patch an installed PRIME-RL checkout to bootstrap trainer LoRA weights from a
PEFT adapter via `PRIME_RL_INIT_ADAPTER=/path/to/adapter`.
"""

from __future__ import annotations

import argparse
from pathlib import Path


TEACHER_LOGPROB_PATCH_OLD = """import asyncio
import time
from concurrent.futures import ThreadPoolExecutor
from itertools import cycle
from pathlib import Path
from typing import Any

import pandas as pd
import verifiers as vf
from rich.console import Console
from rich.table import Table
from verifiers.utils.client_utils import setup_openai_client
"""

TEACHER_LOGPROB_PATCH_NEW = """import asyncio
import os
import time
from concurrent.futures import ThreadPoolExecutor
from itertools import cycle
from pathlib import Path
from typing import Any

import httpx
import pandas as pd
import verifiers as vf
from rich.console import Console
from rich.table import Table
"""

TEACHER_LOGPROB_BLOCK_OLD = """async def compute_teacher_logprobs(
    clients: list[vf.ClientConfig],
    model_name: str,
    samples: list[TrainingSample],
) -> list[list[float]]:
    \"\"\"Compute teacher model logprobs for a batch of training samples via prefill.\"\"\"
    from prime_rl.inference.vllm.serving_generate import GenerateResponse

    async def _compute_single(client_config: vf.ClientConfig, sample: TrainingSample) -> list[float]:
        client = setup_openai_client(client_config)

        response = await client.post(
            \"/generate\",
            cast_to=GenerateResponse,
            body={
                \"model\": model_name,
                \"prompt_token_ids\": sample.prompt_ids + sample.completion_ids,
                \"max_tokens\": 1,
                \"temperature\": 1.0,
                \"top_p\": 1.0,
                \"prompt_logprobs\": True,
            },
        )
        return [0.0 if lp is None else float(lp) for lp in response.prompt_logprobs or []]

    return await asyncio.gather(*[_compute_single(client, sample) for client, sample in zip(cycle(clients), samples)])
"""

TEACHER_LOGPROB_BLOCK_NEW = """async def compute_teacher_logprobs(
    clients: list[vf.ClientConfig],
    model_name: str,
    samples: list[TrainingSample],
) -> list[list[float]]:
    \"\"\"Compute teacher model logprobs for a batch of training samples via prefill.\"\"\"

    async def _compute_single(client_config: vf.ClientConfig, sample: TrainingSample) -> list[float]:
        headers = dict(getattr(client_config, \"extra_headers\", {}) or {})
        api_key_var = getattr(client_config, \"api_key_var\", None)
        if api_key_var:
            api_key = os.getenv(api_key_var)
            if api_key:
                headers.setdefault(\"Authorization\", f\"Bearer {api_key}\")

        async with httpx.AsyncClient(
            base_url=client_config.api_base_url,
            timeout=getattr(client_config, \"timeout\", 1200),
            headers=headers,
        ) as client:
            response = await client.post(
                \"/generate\",
                json={
                    \"model\": model_name,
                    \"prompt_token_ids\": sample.prompt_ids + sample.completion_ids,
                    \"max_tokens\": 1,
                    \"temperature\": 1.0,
                    \"top_p\": 1.0,
                    \"prompt_logprobs\": True,
                },
            )
            response.raise_for_status()
            payload = response.json()

        return [0.0 if lp is None else float(lp) for lp in payload.get(\"prompt_logprobs\") or []]

    return await asyncio.gather(*[_compute_single(client, sample) for client, sample in zip(cycle(clients), samples)])
"""


CALL_MARKER = "        apply_lora_to_model(model, config.lora)\n"
CALL_INSERT = CALL_MARKER + "        _maybe_load_initial_lora_adapter(model)\n"

MODEL_HELPER_ANCHOR = "\n\ndef _patch_qwen3_5_moe_conversion_mapping():\n"
MODEL_HELPER_BLOCK = '''

def _maybe_load_initial_lora_adapter(model: nn.Module) -> None:
    """Load LoRA + modules_to_save weights from PRIME_RL_INIT_ADAPTER if set."""
    adapter_dir = os.environ.get("PRIME_RL_INIT_ADAPTER")
    if not adapter_dir:
        return

    adapter_path = Path(adapter_dir)
    if not adapter_path.exists():
        raise FileNotFoundError(f"PRIME_RL_INIT_ADAPTER does not exist: {adapter_path}")

    weights_path = None
    for candidate in ("adapter_model.safetensors", "adapter_model.bin"):
        path = adapter_path / candidate
        if path.exists():
            weights_path = path
            break
    if weights_path is None:
        raise FileNotFoundError(f"No adapter weights found under {adapter_path}")

    if weights_path.suffix == ".safetensors":
        from safetensors.torch import load_file

        state_dict = load_file(str(weights_path))
    else:
        state_dict = torch.load(weights_path, map_location="cpu")

    remapped_state_dict = {}
    for key, value in state_dict.items():
        new_key = key
        if new_key.startswith("base_model.model."):
            new_key = new_key[len("base_model.model."):]
        new_key = new_key.replace(".modules_to_save.default", "")
        remapped_state_dict[new_key] = value

    incompatible = model.load_state_dict(remapped_state_dict, strict=False)
    logger = get_logger()
    logger.info(
        "Initialized trainer LoRA weights from %s (%d tensors loaded)",
        adapter_path,
        len(remapped_state_dict),
    )
    if getattr(incompatible, "unexpected_keys", None):
        logger.warning("Unexpected adapter keys during bootstrap: %s", incompatible.unexpected_keys)
'''



def patch_ckpt_py(path: Path) -> None:
    text = path.read_text()
    old = """        else:
            # For regular transformers models, revert internal format to original HF hub format
            from transformers.core_model_loading import revert_weight_conversion

            self.logger.debug("Reverting transformers internal format to HF hub format for weight checkpoint")
            start_time = time.perf_counter()
            state_dict = revert_weight_conversion(model, state_dict)
            self.logger.debug(f"Reverted to HF hub format in {time.perf_counter() - start_time:.2f} seconds")
"""
    new = """        else:
            # For regular transformers models, revert internal format to original HF hub format
            try:
                from transformers.core_model_loading import revert_weight_conversion
            except ImportError:
                revert_weight_conversion = None

            if revert_weight_conversion is None:
                self.logger.warning(
                    "transformers.core_model_loading.revert_weight_conversion is unavailable; "
                    "saving the trainer state_dict without that conversion"
                )
            else:
                self.logger.debug("Reverting transformers internal format to HF hub format for weight checkpoint")
                start_time = time.perf_counter()
                state_dict = revert_weight_conversion(model, state_dict)
                self.logger.debug(f"Reverted to HF hub format in {time.perf_counter() - start_time:.2f} seconds")
"""
    if old in text:
        text = text.replace(old, new, 1)
        path.write_text(text)


def patch_orchestrator_utils_py(path: Path) -> None:
    text = path.read_text()
    if (
        "payload.get(\"prompt_logprobs\")" in text
        or "GenerateResponse.model_validate_json(http_response.content)" in text
        or 'cast_to=httpx.Response' in text
    ):
        return
    if TEACHER_LOGPROB_PATCH_OLD not in text:
        raise RuntimeError("Could not find orchestrator imports block to patch")
    if TEACHER_LOGPROB_BLOCK_OLD not in text:
        raise RuntimeError("Could not find teacher logprob block to patch")
    text = text.replace(TEACHER_LOGPROB_PATCH_OLD, TEACHER_LOGPROB_PATCH_NEW, 1)
    text = text.replace(TEACHER_LOGPROB_BLOCK_OLD, TEACHER_LOGPROB_BLOCK_NEW, 1)
    path.write_text(text)


def patch_model_py(path: Path) -> None:
    text = path.read_text()
    changed = False

    if "_maybe_load_initial_lora_adapter(model)" not in text:
        if CALL_MARKER not in text:
            raise RuntimeError("Could not find apply_lora_to_model call in trainer/model.py")
        text = text.replace(CALL_MARKER, CALL_INSERT, 1)
        changed = True

    if "def _maybe_load_initial_lora_adapter(model: nn.Module)" not in text:
        if MODEL_HELPER_ANCHOR not in text:
            raise RuntimeError("Could not find insertion anchor for adapter bootstrap helper in trainer/model.py")
        text = text.replace(MODEL_HELPER_ANCHOR, MODEL_HELPER_BLOCK + MODEL_HELPER_ANCHOR, 1)
        changed = True

    if changed:
        path.write_text(text)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("target", type=Path, help="Path to PRIME-RL repo root or installed prime_rl package dir")
    args = parser.parse_args()

    target = args.target
    candidates = [
        (
            target / "trainer" / "model.py",
            target / "trainer" / "ckpt.py",
            target / "orchestrator" / "utils.py",
        ),
        (
            target / "src" / "prime_rl" / "trainer" / "model.py",
            target / "src" / "prime_rl" / "trainer" / "ckpt.py",
            target / "src" / "prime_rl" / "orchestrator" / "utils.py",
        ),
    ]
    for model_py, ckpt_py, orchestrator_utils_py in candidates:
        if model_py.exists() and ckpt_py.exists() and orchestrator_utils_py.exists():
            break
    else:
        raise FileNotFoundError(f"Could not find PRIME-RL trainer files under {target}")

    patch_model_py(model_py)
    patch_ckpt_py(ckpt_py)
    patch_orchestrator_utils_py(orchestrator_utils_py)


if __name__ == "__main__":
    main()
