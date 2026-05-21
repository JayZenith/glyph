# 2x A100 80GB RLVR Setup (full fine-tune)

Reproducible bring-up for the code-edit RLVR run.

- Trainer on GPU 1 (full-finetunes `JayZenith/GLYPH-SFT-V2`).
- Rollout vLLM on GPU 0 serving the same checkpoint.
- Frozen teacher-anchor vLLM also on GPU 0 (small KL anchor, τ=0.05) serving `JayZenith/GLYPH-SFT-V2`.

Older adapter-era scripts (LoRA + `lm_head` from `JayZenith/glyph-sft-v1-adapter`) live in [`archive_adapter_setup/`](archive_adapter_setup/adapter_setup.md) for historical repro.

## Layout
- Glyph repo: `/workspace/glyph`
- PRIME-RL checkout (separate uv-managed Python 3.12 env): `/workspace/prime-rl-src`

## 1. Install PRIME-RL + patches

```bash
cd /workspace/glyph
bash rl/setup/install_prime_rl.sh
```

What this installs:
- `uv`, Python 3.12, `prime-rl` at `/workspace/prime-rl-src`, submodules.
- Minimal stable Rust toolchain via `rustup` (needed for `cargo`/`rustc` subprocesses).
- flash-attn pinned wheel.
- `rl/setup/patch_install.py` against the installed `prime_rl` and `vllm`.

Patches still relevant in full-FT mode:
- full-weights inference path (`PRIME_RL_INFERENCE_FULL_WEIGHTS=1`).
- `vllm/qwen3.py` packed-weight loader fix (qkv/gate_up). Without this, inference hot-reload after step 1 crashes with `KeyError: 'layers.0.mlp.gate_up_proj.weight'`.
- orchestrator `compute_teacher_logprobs` via raw httpx (used by the teacher anchor).
- ckpt save tolerant of missing `revert_weight_conversion`.

Patches that become no-ops in full-FT mode (kept harmless):
- LoRA + `lm_head` bootstrap from `PRIME_RL_INIT_ADAPTER` (env var simply unset).

## 2. Build the RL prompt set

```bash
python3 -m rl.rust.prepare_cases --phrasings 3 --gold-count 30
```

Writes `runs/rlvr1/prompts.jsonl` and materializes the Cargo project blueprints under `runs/rlvr1/rust_cases/`.

## 3. Smoke test (dry-run config validation)

```bash
bash rl/setup/smoke_test_2xa100.sh
```

Validates that the launcher emits a full-FT config (no LoRA, init from `GLYPH-SFT-V2`, teacher anchor enabled). Exits non-zero on mismatch.

## 4. Run RL

```bash
OUTPUT_DIR=/workspace/glyph/outputs/rlvr1_runN \
  bash rl/setup/run_task_trace_2xa100.sh
```

Defaults: `MODEL=JayZenith/GLYPH-SFT-V2`, `SEQ_LEN=2048`, `MAX_MODEL_LEN=2048`, `MAX_COMPLETION_TOKENS=768`, `MAX_TOOL_ROUNDS=3`, `TEACHER_TAU=0.05`, ports `8000` (rollout) / `8001` (teacher), GPUs `0,1`.

Logs land in `$OUTPUT_DIR/logs/{orchestrator,trainer,inference}.log`. The teacher inference server writes to stdout of the launching shell.

## 5. Verifying the run

- `Step 0` / `Step 1` SUCCESS in orchestrator + trainer logs.
- Teacher server reaches `GET /v1/models = 200` before the first orchestrator step.
- `Orchestrator resumed: checkpoint 1 ready` after step 1 (inference hot-reload survived).
- Average reward trends positive within ~5 steps (bug-fix rollouts that successfully patch + cargo_test contribute ~+2.0 of verifiable signal each).

## Knobs

The launcher is env-var driven:

```bash
MAX_TOOL_ROUNDS=4 MAX_COMPLETION_TOKENS=1024 \
  bash rl/setup/run_task_trace_2xa100.sh
```

Increase `MAX_TOOL_ROUNDS` when bug-fix rollouts need to apply_patch, then cargo_test, then optionally retry.

## Re-running just the patch

```bash
/workspace/prime-rl-src/.venv/bin/python rl/setup/patch_install.py /workspace/prime-rl-src
```

Idempotent; locates `vllm/qwen3.py` via the running Python's import system.
