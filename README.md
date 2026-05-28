# 1093

SFT and RLVR prep for Rust/tool-use traces. The structure rules live in [docs/agent_trace.md](/home/jay-zenith/Desktop/TASK/docs/glyph.md:1).

## Setup on Ampere or Blackwell
```bash
git clone https://github.com/JayZenith/glyph.git
cd glyph
bash sft/setup/install_sft_env.sh
source .venv/bin/activate
hf auth login
```

# Training run on 259 that led to results/eval_formal_post_eval_flatroots.json
```bash
python -m sft.train \
    --model Qwen/Qwen3-4B-Base \
    --tokenizer Qwen/Qwen3-4B-Base \
    --data synthetic_data/signal_259.jsonl \
    --output runs/SIGNAL_259_SFT_E3_LR2E5 \
    --epochs 3 \
    --batch-size 1 \
    --grad-accum 8 \
    --lr 2e-5 \
    --max-seq-length 4096 \
    --save-total-limit 1 \
    --no-train-split
```

# Eval run on 8 creating results/eval_formal_post_eval_flatroots.json
```bash
python -m sft.eval_formal \
    --sft-model runs/SIGNAL_259_SFT_E3_LR2E5/final \
    --train-data synthetic_data/signal_259.jsonl \
    --prompt-section post_eval \
    --output runs/SIGNAL_259_SFT_E3_LR2E5/eval_formal_post_eval_flatroots.json \
    --max-new-tokens 1200 \
    --max-tool-rounds 8 \
    --cases-root runs/rlvr1/rust_cases/eval
```


## Training run for SFT_V1
```bash
python -m sft.train \
  --model Qwen/Qwen3-4B-Base \
  --tokenizer Qwen/Qwen3-4B-Base \
  --data synthetic_data/signal_1062.jsonl \
  --output runs/SIGNAL_1062_SFT_E3_LR2E5 \
  --epochs 3 \
  --batch-size 1 \
  --grad-accum 8 \
  --lr 2e-5 \
  --max-seq-length 4096 \
  --save-total-limit 2 \
  --save-steps 100 \
  --no-train-split
```


Defaults:
```bash
masking_mode: str = "assistant_only"

warmup_ratio: float = 0.03
weight_decay: float = 0.01
lr_scheduler_type: str = "cosine"

bf16: bool = True
tf32: bool = True
gradient_checkpointing: bool = False

save_strategy: str = "steps"
save_steps: int = 500

logging_steps: int = 10
logging_first_step: bool = True
report_to: str = "tensorboard"
```


## EVALS for SFT_V1
1. Rust source similarity audit
```bash
python3 synthetic_data/audit_blueprint_similarity.py \
  --train-data synthetic_data/signal_1062.jsonl \
  --train-blueprints synthetic_data/blueprints \
  --eval-data synthetic_data/eval_heldout_69.jsonl \
  --eval-blueprints synthetic_data/eval_blueprints \
  --max-source-similarity 0.92
```

2. run evals
```bash
python -m sft.eval_formal \
  --sft-model runs/SIGNAL_1062_SFT_E3_LR2E5/final \
  --train-data synthetic_data/signal_1062.jsonl \
  --prompt-file sft/evals/eval_prompts_heldout_69.yaml \
  --prompt-section post_eval_heldout_69 \
  --output runs/SIGNAL_1062_SFT_E3_LR2E5/eval_formal_heldout_69.json \
  --max-new-tokens 4000 \
  --max-tool-rounds 15 \
  --cases-root runs/rlvr1/rust_cases/eval_heldout_69
```
<!--
```bash
python -m sft.eval_test_loss \
  --base Qwen/Qwen3-4B-Base \
  --sft runs/GLYPH_SFT_FINAL/final \
  --test-set runs/GLYPH_SFT_FINAL/test_set \
  --output runs/GLYPH_SFT_FINAL/eval_test_loss.json
```-->


## Key Results
- SFT_V1 model: `JayZenith/SFT_V1`
- SFT_V1 dataset: `JayZenith/SFT_V1_DATASET`
- Eval: `52 / 69` valid traces on held-out real Rust/tool eval.
- Terminal tool success: `68 / 69`.
- Result/call ID match, no repetition, and no truncation: `100%`.
- Rust source similarity audit against train set: `0` exact duplicates, `0` near duplicates at threshold `0.92`.

Remaining failures:
- Main failure mode is termination: `17` traces kept using tools until max rounds and did not emit `FINAL`.
- Only `1` trace had terminal tool/task failure.
- This points to RLVR as the next step: reward successful verifier result followed by exactly one clean `FINAL`.

## Notes
- reported repo commit of SFT_V1: `5cb4699b1f9d544207315d551b125f672051c76c`

## RLVR Handoff
- Start from HF model `JayZenith/SFT_V1` trained on `synthetic_data/signal_1062.jsonl`.
- Use held-out eval `sft/evals/eval_prompts_heldout_69.yaml` with blueprints in `synthetic_data/eval_blueprints`.
- Main target: improve clean termination after verifier success. SFT already gets `68 / 69` terminal tool success, but only `52 / 69` valid final traces.
- Reward should favor real verifier success followed by exactly one clean `FINAL`, and penalize invalid schema, extra tool loops, max-round exhaustion, and task failure.
