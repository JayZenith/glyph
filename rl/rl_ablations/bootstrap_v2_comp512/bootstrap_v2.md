# Bootstrap v2 Task-Trace RL

Current good-signal run:

```bash
OUTPUT_DIR=/workspace/glyph/rl_ablations/bootstrap_v2_comp512 \
SEQ_LEN=4096 \
MAX_MODEL_LEN=4096 \
MAX_TOOL_ROUNDS=1 \
MAX_COMPLETION_TOKENS=512 \
STOP_ON_RESULT=1 \
bash setup/run_task_trace_2xa100.sh
```

Key settings:

- `STOP_ON_RESULT=1` stops assistant generation before result-like text so the env emits real tool results.
- Reserved Qwen token IDs above tokenizer length are masked with `logit_bias=-100`.
- Fake assistant-authored results are capped at `-0.25`.
- Clean assistant/tool boundary gets `+1.5`.

Latest local snapshot was pulled from the remote run at about step 54.

```text
summary latest10 avg_reward 1.8778 avg_pos 35.9 avg_no_call 3.1 avg_fake 8.7 avg_tools 46.5 avg_len 1272
```

Interpretation:

- Real RL signal is present: positive rewards are stable across most rollouts.
- `posfake=0`, so fake result hallucinations are not getting positive reward.
- Tool use remains healthy: latest10 average tools is about `46/48`.
- `no_call` exists but is not currently runaway.
- Verbosity still needs future tuning; track `len`.

Inspect locally:

```bash
python3 rl/scripts/inspect_rollouts.py rl/rl_ablations/bootstrap_v2_comp512
```

Artifacts copied locally for manual inspection:

- `rl/rl_ablations/bootstrap_v2_comp512/logs/`
- `rl/rl_ablations/bootstrap_v2_comp512/rollouts/`

Those heavy artifacts are intentionally ignored by `rl/rl_ablations/.gitignore`.
