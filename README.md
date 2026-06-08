# SFT → RLVR for a Rust tool-use agent



**SFT (SFT_V1 recipe):** but make sure we train above the max sequence length within the dataset!
```bash
python -m sft.train \
  --model Qwen/Qwen3-4B-Base --tokenizer Qwen/Qwen3-4B-Base \
  --data synthetic_data/signal_1062.jsonl \
  --output runs/SIGNAL_1062_SFT_E3_LR2E5 \
  --epochs 3 --batch-size 1 --grad-accum 8 --lr 2e-5 \
  --max-seq-length 4096 --no-train-split
# deep data (v2/v3): --max-seq-length 11000 --gradient-checkpointing  (traces reach ~10.4k tok)
```

**Held-out eval (any SFT or RLVR checkpoint):**
```bash
python -m sft.eval_formal --sft-model runs/SIGNAL_1062_SFT_E3_LR2E5/final \
  --prompt-file sft/evals/eval_prompts_heldout_69.yaml --prompt-section post_eval_heldout_69 \
  --cases-root runs/rlvr1/rust_cases/eval_heldout_69 \
  --output out.json --max-new-tokens 4000 --max-tool-rounds 20
```

**pass@k scan (the diagnostic — may run this before any RLVR):**
```bash
python sft/passk_scan_vllm.py --sft-model JayZenith/SFT_V1 \
  --prompt-file runs/rlvr_passk_train150/prompts.yaml --prompt-section train_passk_scan_134 \
  --cases-root runs/rlvr_passk_train150/cases -k 8 --temperature 0.8 \
  --output results/passk_train134.json
# band: 0<solves<k = rlvr-target (gradient) | ==k = solved | ==0 = capability-gap
```

> **vLLM scan gotcha (cost us a day):** vLLM defaults to `skip_special_tokens=True`, which
> strips `<|im_end|>` from the output text. A *string* stop on `"<|im_end|>"` then never
> matches, generation runs past every turn boundary, no tools execute, and the scan reports
> **0/8 on prompts the model actually solves**. Stop on the token **id**
> (`tokenizer.convert_tokens_to_ids("<|im_end|>")`) and re-append `<|im_end|>` so the
> protocol parser can segment turns. Always compare before/after on the **same** engine.

**RLVR example run:**

Training environment:
```bash
git clone https://github.com/JayZenith/glyph.git && cd glyph
bash rl/setup/install_prime_rl.sh
source /workspace/prime-rl-src/.venv/bin/activate
```

Example  launch:
```bash
python rl/train.py \
  --data synthetic_data/rl_prompts_heldout69_passk_targets.jsonl \
  --output outputs/rlvr_heldout69_passk8_targets_seq8192_mlen12288_train2_memfix \
  --max-steps 50 \
  --batch-size 8 \
  --rollouts-per-example 8 \
  --seq-len 8192 \
  --max-model-len 12288 \
  --teacher-max-model-len 12288 \
  --max-completion-tokens 1536 \
  --learning-rate 2e-7 \
  --checkpoint-interval 25 \
  --temperature 0.8 \
  --max-tool-rounds 15 \
  --tool-timeout 30 \
  --teacher-tau 0.4 \
  --activation-checkpointing \
  --fused-lm-head-token-chunk-size auto \
  --prime-rl-gpu-ids 0,1,2 \
  --num-infer-gpus 1 \
  --num-train-gpus 2 \
  --gpus-per-node 3 \
  --port 8000 \
  --teacher-device 3
```

This was run on a 4xA100 instance. GPUs `0,1,2` were assigned to PRIME-RL
(trainer + inference); GPU `3` was reserved for the frozen teacher via
`--teacher-device 3`, so it is intentionally outside `--prime-rl-gpu-ids`.


Matched pass@8 eval on checkpoint 25:
```bash
NAMES=$(paste -sd, synthetic_data/heldout69_passk_target_names.txt)
PYTHONPATH=/workspace/glyph python sft/passk_scan_vllm.py \
  --sft-model outputs/rlvr_heldout69_passk8_targets_seq8192_mlen12288_train2_memfix/weights/step_25 \
  --prompt-file sft/evals/eval_prompts_heldout_69.yaml \
  --prompt-section post_eval_heldout_69 \
  --cases-root synthetic_data/eval_blueprints \
  --names "$NAMES" \
  -k 8 \
  --temperature 0.8 \
  --max-new-tokens 4000 \
  --max-tool-rounds 15 \
  --tool-timeout 30 \
  --max-model-len 12288 \
  --dtype bfloat16 \
  --output results/RLVR_HELDOUT69_PASSK_STEP25/passk8_step25_8targets.json \
  --no-resume
```

Result:
```text
SFT_V1:   47/64 = 0.734
step_25:  54/64 = 0.844
delta:    +7 solves, +10.9 pts
```

Full original held-out 69 formal eval on the same checkpoint:
```bash
python -m sft.eval_formal \
  --sft-model JayZenith/RLVR_HELDOUT69_PASSK_STEP25 \
  --prompt-file sft/evals/eval_prompts_heldout_69.yaml \
  --prompt-section post_eval_heldout_69 \
  --train-data synthetic_data/signal_1062.jsonl \
  --cases-root runs/rlvr1/rust_cases/eval_heldout_69 \
  --output results/RLVR_HELDOUT69_FORMAL_STEP25/eval_formal_heldout_69_step25.json \
  --max-new-tokens 4000 \
  --max-tool-rounds 15 \
  --tool-timeout 30
```

## Repo map

```
synthetic_data/   data generation + datasets (data_lineage.md), pass@k band, RL prompts
sft/              SFT train (train.py), eval (eval_formal.py), pass@k scan (passk_scan*.py)
rl/               PRIME-RL env + reward (task_trace.py), configs/, train.py wrapper, RLVR_NOTES.md
agent_runtime/    protocol parser + real cargo execution / per-rollout sandboxing
results/          eval JSON + tensorboard/W&B/logs/rollouts, including final step-25 artifact
blog/             blog_copy_copy.md (current narrative + primer), make_figures.py, assets/*.png
reward_golden_tests.py   reward unit tests (solving dominates, bounded, format floor)
```

## Datasets (`synthetic_data/`)

GPT-authored task specs → materialized into **real crates** → kept only if real tool
execution matched the intended trajectory (`data_lineage.md`).

- `signal_1062.jsonl` — 1062 traces; all recovery is depth-1 (one fail, one fix).
- `signal_v2_1323.jsonl` — + 261 variable-depth (1–5) recovery cases.
- `signal_v3.jsonl` — + 199 depth-3..5 cases, oversampling clean `PASS→FINAL` (2083 rows / 1522 unique).
- `rl_prompts_passk_target.jsonl` — the 39 rlvr-target prompts (the RLVR_PASSK band).
- `rl_prompts_heldout69_passk_targets.jsonl` — the final 8 mixed held-out-failure prompts.

Train↔eval audited at 0.92 source similarity, 0 near-duplicates
(`synthetic_data/audit_blueprint_similarity.py`).
