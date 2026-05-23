# RLVR on GLYPH_SFT (full fine-tune) — manual repro

Reproducible bring-up for code-edit RLVR on top of `JayZenith/GLYPH_SFT`.
Target: 2× A100 80GB (or 2× H100 80GB / 2× H200). Trainer on GPU 1, rollout
vLLM + frozen teacher vLLM on GPU 0.

## 1. Clone

```bash
mkdir -p /workspace && cd /workspace
git clone https://github.com/JayZenith/glyph.git
cd glyph
```

## 2. Install prime-rl + flash-attn + rust + patches

```bash
bash rl/setup/install_prime_rl.sh
```

What it does:
- Installs `uv`, Python 3.12, clones `prime-rl` into `/workspace/prime-rl-src`.
- Inits `verifiers`, `renderers`, `research-environments`, `pydantic-config` submodules.
- `uv sync` against the prime-rl workspace.
- Installs a pinned flash-attn wheel for torch 2.11 / cu128 / cp312.
- Installs a stable Rust toolchain (`cargo`, `rustc`) — needed for the verifier subprocesses.
- Runs `rl/setup/patch_install.py`. Load-bearing in full-FT mode:
  - vLLM Qwen3 packed-weight loader (avoids `KeyError: 'layers.0.mlp.gate_up_proj.weight'` on hot-reload).
  - `compute_teacher_logprobs` via raw httpx (used by the KL anchor).
  - Full-weights inference broadcast.
  - ckpt save tolerant of missing `revert_weight_conversion`.
  - The LoRA-bootstrap patch is a no-op (it gates on `PRIME_RL_INIT_ADAPTER`, which the full-FT launcher never sets).

## 3. Fetch GLYPH_SFT

```bash
source /workspace/prime-rl-src/.venv/bin/activate
export HF_HOME=/workspace/.hf_home
python -c "from huggingface_hub import snapshot_download; print(snapshot_download('JayZenith/GLYPH_SFT'))"
```

## 4. Build the RL prompt set

```bash
cd /workspace/glyph
python -m rl.rust.prepare_cases --phrasings 3 --gold-count 12
```

Writes `runs/rlvr1/prompts.jsonl` (~95 rows by default: 83 Rust execution / 12 targeted structure) and materializes the Cargo project blueprints under `runs/rlvr1/rust_cases/`. The structure slice is mined from the final SFT dataset and targets the known failure modes: response-tail hygiene, todo closure, and patch-then-verify completion.

## 5. Smoke test (config dry-run)

```bash
bash rl/setup/smoke_test_2xa100.sh
```

Validates: full-FT trainer config (no LoRA section), init model `JayZenith/GLYPH_SFT`, teacher anchor enabled, 2-GPU layout. Exits non-zero on mismatch.

## 6. Run RL

```bash
export HF_HOME=/workspace/.hf_home
export CARGO_HOME=$HOME/.cargo
export RUSTUP_HOME=$HOME/.rustup
export PATH=$CARGO_HOME/bin:$PATH

OUTPUT_DIR=/workspace/glyph/outputs/rlvr1_run1 \
  bash rl/setup/run_task_trace_2xa100.sh
```

Defaults (override via env):
- `MODEL=JayZenith/GLYPH_SFT`
- `HW_PROFILE=auto` — auto-detects from `nvidia-smi`. Picks one of:
  - `a100-80gb` (also used for H100): `SEQ_LEN=5120`, `MAX_COMPLETION_TOKENS=1536`, `MAX_TOOL_ROUNDS=5`. Trainer peaks ~78 GiB.
  - `blackwell-96gb` (RTX PRO 6000 SE / H200 / B200): `SEQ_LEN=6144`, `MAX_COMPLETION_TOKENS=1536`, `MAX_TOOL_ROUNDS=5`. Trainer peaks ~88-90 GiB.
  - Any individual knob (`SEQ_LEN=...`, `MAX_TOOL_ROUNDS=...`) overrides the preset.
- Rollout port `8000`, GPUs `0,1`.
- `TEACHER_ANCHOR=0` by default. The pinned prime-rl version forbids
  `orchestrator.teacher` in `training_mode='rl'`; teacher anchor requires
  switching to `'opd'` (online policy distillation) mode, which is a TODO
  for this launcher.

Logs land in `$OUTPUT_DIR/logs/{orchestrator,trainer,inference}.log`. The teacher inference server's output goes to the launching shell's stdout.

## 7. Watch the run

```bash
python rl/scripts/inspect_rollouts.py $OUTPUT_DIR --tail 20
```

Healthy signals:
- `Step 0` / `Step 1` SUCCESS in orchestrator + trainer logs.
- Teacher inference: `GET /v1/models = 200` before step 0.
- `Orchestrator resumed: checkpoint 1 ready` after step 1 (inference hot-reload survived).
- Average reward trending positive within ~5 orchestrator steps.

Bug-fix rollouts that successfully patch + cargo_test contribute ~+2.0 of verifiable signal each, on top of structure (+1.0) and tool alignment.

## Notes

- The dataset (`runs/`) and outputs (`outputs/`) are gitignored. Regenerate on every fresh box.
- Older adapter-era scripts live under `rl/setup/archive_adapter_setup/` for historical reproduction only — not used in this path.
