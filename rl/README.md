## PRIME-RL Integration

This project now delegates all RL fine-tuning to [PRIME-RL](https://github.com/PrimeIntellect-ai/prime-rl). The legacy GRPO trainer has been removed in favour of PRIME-RL’s orchestrator/trainer/inference stack.

### Setup

1. Install local glyph-side training and audit deps:
   ```bash
   pip install -r requirements-train.txt
   ```

2. Install PRIME-RL in its own upstream-managed environment:
   ```bash
   bash rl/setup_prime_rl.sh
   ```
   This clones upstream `prime-rl`, creates its `uv` / Python 3.12 environment, installs `peft`, installs a pinned `flash-attn` wheel for the current PRIME-RL torch/CUDA stack, and applies the local adapter-bootstrap patch needed for continuing from `JayZenith/glyph-sft-v1-adapter`.

   If the PRIME-RL stack changes and the pinned wheel no longer matches, set:
   ```bash
   FLASH_ATTN_WHEEL_URL=...
   bash rl/setup_prime_rl.sh
   ```
   The script intentionally fails fast instead of silently compiling `flash-attn` from source.

3. (Optional) Generate/refresh prompts:
   ```bash
   uv run python generate_prompts.py --count 2000 --output traces.processed.jsonl
   ```

Do not install PRIME-RL through `requirements-train.txt`. Its upstream dependency stack is resolved through its own `pyproject.toml` and `uv` environment, and trying to flatten that into this repo's requirements file is brittle.

### Local Environment

- The TASK verifier/reward logic now lives in `task_format/`.
- The PRIME-RL verifiers environment entrypoint is `task_trace.load_environment`.
- Config templates are under `configs/prime_rl/task_trace/`.

### Launching RL

Use the new wrapper which configures PRIME-RL with the local environment:

```bash
uv run python rl_train.py \
  --model Qwen/Qwen3-4B \
  --data traces.processed.jsonl \
  --output outputs \
  --max-steps 400 \
  --batch-size 192 \
  --rollouts-per-example 12 \
  --seq-len 4096
```

Flags such as `--max-samples`, `--max-trace-chars`, `--max-tokens`, etc. forward directly to the PRIME-RL orchestrator/trainer. Use `--dry-run` to print the command without launching, and append extra PRIME-RL arguments after `--` to pass them straight through.

### Direct PRIME-RL Invocation

Advanced users can interact with PRIME-RL directly:

```bash
uv run rl \
  --trainer @ configs/prime_rl/task_trace/trainer.toml \
  --orchestrator @ configs/prime_rl/task_trace/orchestrator.toml \
  --inference @ configs/prime_rl/task_trace/inference.toml \
  --orchestrator.env.0.args.data_path traces.processed.jsonl
```

This is equivalent to what `rl_train.py` builds automatically.
