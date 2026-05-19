# Task-Trace RL Phases

## Current Remote

- Instance: `root@34.44.8.118`, SSH port `21349`
- Repo path: `/workspace/glyph`
- PRIME-RL path: `/workspace/prime-rl-src`
- Commit: `e4d2020 Fix Rust task-trace RL bootstrap`
- Current run output: `/workspace/glyph/outputs/task_trace_rl_phase2_no_stop_from25_v7`
- Keep on instance: model caches, W&B offline runs, rollout logs, checkpoints, generated outputs.

## Mirrored Source State

Local and instance tracked source/doc changes are mirrored:

```bash
git status --short
# D rl/rl_ablations/max_chars500/maxchars.md
# M setup/install_prime_rl.sh
# M rl/task_trace.py
# M rl/train.py
# ?? rl/rl_ablations/phases/phases.md
# ?? rl/scripts/live_rollout_viewer.py
```

`setup/install_prime_rl.sh` now tests whether the temp directory is executable before using it.

## Prior Baseline

Local prior rollout summary:

```bash
python3 rl/scripts/inspect_rollouts.py rl/rl_ablations/bootstrap_v2_comp512 --tail 20
```

Latest observed summary:

```text
summary latest10 avg_reward 1.9358 avg_pos 36.7 avg_no_call 3.9 avg_fake 7.8 avg_tools 46.1 avg_len 1272
```

## Phase 1: Stop-On-Result Bootstrap

Goal: short bootstrap with one tool round and hard stop before assistant-authored results.

Local compact archive path before remote deletion:

```text
rl/rl_ablations/phases/phase1_stop_result_4096/
```

Saved locally:

```text
configs/
dry_run_config.json
metrics.txt
manifest.txt
checkpoints.txt
tree_dirs.txt
rollouts/step_0..step_34/train_rollouts.jsonl
rollouts/step_0..step_34/train_rollouts.bin
```

Not saved: large remote logs, weights, checkpoints.

Dry-run config command:

```bash
cd /workspace/glyph
export HF_HOME=/workspace/.hf_home TMPDIR=/workspace/.tmp
export PYTHONPATH=/workspace/glyph:/workspace/glyph/rl:${PYTHONPATH:-}
source /workspace/prime-rl-src/.venv/bin/activate
python3 rl/train.py \
  --adapter JayZenith/glyph-sft-v1-adapter \
  --rollout-init-model JayZenith/glyph-sft-v1 \
  --port 8000 \
  --data runs/rl1/rust_tool_prompts_8.jsonl \
  --output outputs/task_trace_rl_phase1_stop_result_4096 \
  --gpu-memory-utilization 0.7 \
  --seq-len 4096 \
  --max-model-len 4096 \
  --max-completion-tokens 512 \
  --max-tool-rounds 1 \
  --stop-on-result \
  --max-steps 100 \
  --checkpoint-interval 25 \
  --dump-config outputs/task_trace_rl_phase1_stop_result_4096/dry_run_config.json \
  --dry-run
```

Detached launch command:

```bash
cd /workspace/glyph
nohup bash -lc 'export HF_HOME=/workspace/.hf_home TMPDIR=/workspace/.tmp TEMP=/workspace/.tmp TMP=/workspace/.tmp CUDA_VISIBLE_DEVICES=0,1 CARGO_HOME=${CARGO_HOME:-$HOME/.cargo} RUSTUP_HOME=${RUSTUP_HOME:-$HOME/.rustup}; export PATH="$CARGO_HOME/bin:$PATH"; export PYTHONPATH=/workspace/glyph:/workspace/glyph/rl:${PYTHONPATH:-}; source /workspace/prime-rl-src/.venv/bin/activate; exec python3 rl/train.py --adapter JayZenith/glyph-sft-v1-adapter --rollout-init-model JayZenith/glyph-sft-v1 --port 8000 --data runs/rl1/rust_tool_prompts_8.jsonl --output outputs/task_trace_rl_phase1_stop_result_4096 --gpu-memory-utilization 0.7 --seq-len 4096 --max-model-len 4096 --max-completion-tokens 512 --max-tool-rounds 1 --stop-on-result --max-steps 100 --checkpoint-interval 25' \
  > /workspace/glyph/outputs/task_trace_rl_phase1_stop_result_4096/train.log 2>&1 &
```

Important config confirmed:

- `trainer.max_steps = 100`
- `trainer.ckpt.interval = 25`
- `orchestrator.seq_len = 4096`
- `inference.model.max_model_len = 4096`
- `max_completion_tokens = 512`
- `max_tool_rounds = 1`
- stop strings include `result {`, `result //`, `\nresult`

Monitor commands:

```bash
ssh -p 21349 root@34.44.8.118 'nvidia-smi --query-gpu=index,memory.used,utilization.gpu --format=csv,noheader'
ssh -p 21349 root@34.44.8.118 'tail -n 120 /workspace/glyph/outputs/task_trace_rl_phase1_stop_result_4096/logs/orchestrator.log'
ssh -p 21349 root@34.44.8.118 'tail -n 120 /workspace/glyph/outputs/task_trace_rl_phase1_stop_result_4096/logs/inference.log'
ssh -p 21349 root@34.44.8.118 'tail -n 120 /workspace/glyph/outputs/task_trace_rl_phase1_stop_result_4096/logs/trainer.log'
```

Live terminal viewer:

```bash
cd /workspace/glyph
python3 rl/scripts/live_rollout_viewer.py outputs/task_trace_rl_phase1_stop_result_4096 --terminal --interval 5
```

Observed startup:

- PRIME-RL launched inference on GPU 0 and trainer on GPU 1.
- Trainer loaded `Qwen/Qwen3-4B-Base` with LoRA rank 64, alpha 64, `lm_head` in `modules_to_save`.
- Inference loaded `JayZenith/glyph-sft-v1` with `max_model_len=4096`.
- Orchestrator initially waited for inference readiness while vLLM loaded shards.
- Inference became healthy at `GET /v1/models = 200`.
- Do not preserve interim metrics here; record the step 50 checkpoint/metrics for review.

## Phase 2: No-Stop Continuation

Phase 2 was started from the phase 1 `step_25` checkpoint. The checkpoint was hardlinked into a separate output directory to keep phase 1 intact:

```bash
cd /workspace/glyph
mkdir -p outputs/task_trace_rl_phase2_no_stop_from25/checkpoints \
  outputs/task_trace_rl_phase2_no_stop_from25/weights \
  outputs/task_trace_rl_phase2_no_stop_from25/run_default/checkpoints \
  outputs/task_trace_rl_phase2_no_stop_from25/run_default/broadcasts
cp -al outputs/task_trace_rl_phase1_stop_result_4096/checkpoints/step_25 \
  outputs/task_trace_rl_phase2_no_stop_from25/checkpoints/
cp -al outputs/task_trace_rl_phase1_stop_result_4096/weights/step_25 \
  outputs/task_trace_rl_phase2_no_stop_from25/weights/
cp -al outputs/task_trace_rl_phase1_stop_result_4096/run_default/checkpoints/step_25 \
  outputs/task_trace_rl_phase2_no_stop_from25/run_default/checkpoints/
cp -al outputs/task_trace_rl_phase1_stop_result_4096/run_default/broadcasts/step_25 \
  outputs/task_trace_rl_phase2_no_stop_from25/run_default/broadcasts/
```

Dry run:

```bash
python3 rl/train.py \
  --adapter JayZenith/glyph-sft-v1-adapter \
  --rollout-init-model JayZenith/glyph-sft-v1 \
  --port 8000 \
  --data runs/rl1/rust_tool_prompts_8.jsonl \
  --output outputs/task_trace_rl_phase2_no_stop_from25 \
  --gpu-memory-utilization 0.7 \
  --seq-len 4096 \
  --max-model-len 4096 \
  --max-completion-tokens 512 \
  --max-tool-rounds 1 \
  --max-steps 50 \
  --checkpoint-interval 25 \
  --resume-step 25 \
  --dump-config outputs/task_trace_rl_phase2_no_stop_from25/dry_run_config.json \
  --dry-run
```

Detached launch:

```bash
cd /workspace/glyph
nohup bash -lc 'export HF_HOME=/workspace/.hf_home TMPDIR=/workspace/.tmp TEMP=/workspace/.tmp TMP=/workspace/.tmp CUDA_VISIBLE_DEVICES=0,1 CARGO_HOME=${CARGO_HOME:-$HOME/.cargo} RUSTUP_HOME=${RUSTUP_HOME:-$HOME/.rustup}; export PATH="$CARGO_HOME/bin:$PATH"; export PYTHONPATH=/workspace/glyph:/workspace/glyph/rl:${PYTHONPATH:-}; source /workspace/prime-rl-src/.venv/bin/activate; exec python3 rl/train.py --adapter JayZenith/glyph-sft-v1-adapter --rollout-init-model JayZenith/glyph-sft-v1 --port 8000 --data runs/rl1/rust_tool_prompts_8.jsonl --output outputs/task_trace_rl_phase2_no_stop_from25 --gpu-memory-utilization 0.7 --seq-len 4096 --max-model-len 4096 --max-completion-tokens 512 --max-tool-rounds 1 --max-steps 50 --checkpoint-interval 25 --resume-step 25' \
  > /workspace/glyph/outputs/task_trace_rl_phase2_no_stop_from25/train.log 2>&1 &
```

Confirmed config:

- `--stop-on-result` removed
- `resume_step = 25`
- `max_steps = 50`
- `checkpoint_interval = 25`
- `max_tool_rounds = 1`
- fake-result penalty still active in `rl/task_trace.py`
- same Rust prompt data
- compare against phase 1 and prior baseline

Compare metrics:

```text
reward, pos, posfake, no_call, fake, tools, len
```

### Phase 2 Resume Fix

Failed `v1`-`v5` attempts hit `POST /inference/v1/generate 404` after resume. PRIME-RL regenerated orchestrator LoRA name as `r64-a64.0`, then routed scheduler requests to that model name.

`rl/train.py` now always serves these aliases:

```text
JayZenith/glyph-sft-v1
Qwen/Qwen3-4B-Base
JayZenith__glyph-sft-v1-adapter-r64-a64
r64-a64.0
```

`v6` confirmed the 404 was fixed, but behavior was bad:

```text
latest10 avg_reward -0.3562 avg_pos 1 avg_no_call 3.25 avg_fake 44.75 avg_tools 92.5 avg_len 3520
```

`rl/task_trace.py` now makes assistant-authored fake results a hard failure:

```text
FAKE_RESULT_REWARD = -2.0
```

Old phase2 output dirs were removed to save disk:

```bash
rm -rf outputs/task_trace_rl_phase2_no_stop_from25*
```

Phase2 `v7` launch:

```bash
cd /workspace/glyph
OUT=outputs/task_trace_rl_phase2_no_stop_from25_v7
SRC=outputs/task_trace_rl_phase1_stop_result_4096
mkdir -p "$OUT/checkpoints" "$OUT/weights" "$OUT/run_default/checkpoints" "$OUT/run_default/broadcasts"
cp -al "$SRC/checkpoints/step_25" "$OUT/checkpoints/"
cp -al "$SRC/weights/step_25" "$OUT/weights/"
cp -al "$SRC/run_default/checkpoints/step_25" "$OUT/run_default/checkpoints/"
cp -al "$SRC/run_default/broadcasts/step_25" "$OUT/run_default/broadcasts/"
nohup bash -lc 'export HF_HOME=/workspace/.hf_home TMPDIR=/workspace/.tmp TEMP=/workspace/.tmp TMP=/workspace/.tmp CUDA_VISIBLE_DEVICES=0,1 CARGO_HOME=${CARGO_HOME:-$HOME/.cargo} RUSTUP_HOME=${RUSTUP_HOME:-$HOME/.rustup}; export PATH="$CARGO_HOME/bin:$PATH"; export PYTHONPATH=/workspace/glyph:/workspace/glyph/rl:${PYTHONPATH:-}; source /workspace/prime-rl-src/.venv/bin/activate; exec python3 rl/train.py --adapter JayZenith/glyph-sft-v1-adapter --rollout-init-model JayZenith/glyph-sft-v1 --port 8000 --data runs/rl1/rust_tool_prompts_8.jsonl --output outputs/task_trace_rl_phase2_no_stop_from25_v7 --gpu-memory-utilization 0.7 --seq-len 4096 --max-model-len 4096 --max-completion-tokens 512 --max-tool-rounds 1 --max-steps 50 --checkpoint-interval 25 --resume-step 25' \
  > "$OUT/train.log" 2>&1 &
```

### Phase 2 v8 Guarded Continuation

`v7` proved hard fake-result reward alone was not enough:

```text
step 25 avg_reward -1.7279 pos 2 fake 44 tools 96 len 3502
```

`v8` resumed from step 25 with `--stop-on-result` restored. This is guarded phase2, not pure no-stop, but it clamps generations before assistant-authored result blocks and lets the env provide tool messages.

Observed early metrics:

```text
step 25 avg 1.4418 pos 34 fake 10 tools 44 len 1104
step 26 avg 2.1117 pos 42 fake 2 tools 44 len 1220
```

Final phase2 `v8` review:

```text
checkpoint: step_50 present
rollouts: step_25..step_49
latest10 avg_reward 1.6423 avg_pos 36.6 avg_no_call 3 avg_fake 7.5 avg_tools 45.8 avg_len 1247
step_49: avg 1.8279 pos 40 fake 6 no_call 1 tools 46 len 1238
```

Qualitative step 49 sample:

```text
good clean tool-call trajectories: 40/48
fake result failures: 4/48 by direct sample parser, 6/48 by inspect script
no act call: 1/48
assistant response-after-result residue: 4/48
```

Assessment: successful guarded bridge phase. It teaches real `act { call ... }` behavior and avoids the v7 collapse, but it is not proof of pure no-stop behavior because `--stop-on-result` is active and `max_tool_rounds=1` mostly trains call-and-wait, not final-answer-after-tool.

Compact local archive:

```text
rl/rl_ablations/phases/phase2_stop_result_from25_v8/
```

Saved locally:

```text
configs/
metrics.txt
manifest.txt
files_final.txt
checkpoints.txt
orchestrator_tail.txt
trainer_tail.txt
rollouts/step_25..step_49/train_rollouts.jsonl
rollouts/step_25..step_49/train_rollouts.bin
```

Remote cleanup for phase3:

```text
deleted: phase1 remote output
deleted: v8 step_25 checkpoint/weights/broadcast
deleted: v8 step_49 broadcast
deleted: v8 rollouts/logs/wandb
kept: v8 step_50 checkpoints, weights, orchestrator checkpoint, broadcast
remote free: ~47G on /workspace
```

Current `v8` launch includes:

```bash
python3 rl/train.py \
  --adapter JayZenith/glyph-sft-v1-adapter \
  --rollout-init-model JayZenith/glyph-sft-v1 \
  --port 8000 \
  --data runs/rl1/rust_tool_prompts_8.jsonl \
  --output outputs/task_trace_rl_phase2_no_stop_from25_v8 \
  --gpu-memory-utilization 0.7 \
  --seq-len 4096 \
  --max-model-len 4096 \
  --max-completion-tokens 512 \
  --max-tool-rounds 1 \
  --stop-on-result \
  --max-steps 50 \
  --checkpoint-interval 25 \
  --resume-step 25
```
