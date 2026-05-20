# glyph

Reproduction notes for the successful `GLYPH_SFT_OFFICIAL_V1` supervised fine-tune.

## Outcome

- Base model: `Qwen/Qwen3-4B-Base`
- Dataset: `synthetic_data/glyph_gold50/gold_glyph_2500.jsonl`
- Split: `2000 / 250 / 250`
- Final usable checkpoint: `runs/sft_toolturn_v1_fullft1/checkpoint-250`
- HF model: `JayZenith/GLYPH_SFT`

## Important Knobs

- Full fine-tune, not LoRA: `--no-use-lora`
- Epochs: `1`
- Trunk LR: `1e-5`
- `lm_head` LR: `1.5e-5`
- Loss masking: `assistant_only`
- Max sequence length: `1024`
- Batch size: `1`
- Gradient accumulation: `8`
- Save steps: `100`
- Gradient checkpointing: on
- BF16: on

## Train

```bash
cd /workspace/glyph
git pull --ff-only
source .venv/bin/activate
python -m sft.train \
  --model Qwen/Qwen3-4B-Base \
  --data synthetic_data/glyph_gold50/gold_glyph_2500.jsonl \
  --output runs/sft_toolturn_v1_fullft1 \
  --epochs 1 \
  --no-use-lora \
  --lr 1e-5 \
  --lm-head-lr 1.5e-5 \
  --save-steps 100 2>&1 | tee runs/sft_toolturn_v1_fullft1.log
```

## Eval

Held-out test loss:

```bash
cd /workspace/glyph
source .venv/bin/activate
python -m sft.eval_test_loss \
  --base Qwen/Qwen3-4B-Base \
  --sft runs/sft_toolturn_v1_fullft1/checkpoint-250 \
  --test-set runs/sft_toolturn_v1_fullft1/test_set \
  --output runs/sft_toolturn_v1_fullft1/eval_test_loss.json
```

Formal eval, 2 prompts:

```bash
python -m sft.eval_formal \
  --sft-model runs/sft_toolturn_v1_fullft1/checkpoint-250 \
  --output runs/sft_toolturn_v1_fullft1/eval_formal_limit2.json \
  --limit 2 \
  --max-new-tokens 1200 \
  --max-tool-rounds 4
```

Formal eval, 8 prompts:

```bash
python -m sft.eval_formal \
  --sft-model runs/sft_toolturn_v1_fullft1/checkpoint-250 \
  --output runs/sft_toolturn_v1_fullft1/eval_formal_limit8.json \
  --limit 8 \
  --max-new-tokens 1200 \
  --max-tool-rounds 4
```

Formal eval, 18 prompts:

```bash
python -m sft.eval_formal \
  --sft-model runs/sft_toolturn_v1_fullft1/checkpoint-250 \
  --output runs/sft_toolturn_v1_fullft1/eval_formal_limit18.json \
  --limit 18 \
  --max-new-tokens 1200 \
  --max-tool-rounds 4
```

## Results

- Held-out weighted loss: `2.2446 -> 0.3284`
- Held-out perplexity: `9.44 -> 1.39`
- Formal eval, 8 prompts: `7/8` valid
- Formal eval, 18 prompts: `17/18` valid

## Notes

- The single 18-prompt failure was `ci_timeout_fallback`.
- That failure looked like a mock tool/result tag mismatch, not a general format failure.
- This checkpoint is the one to carry forward into RLVR.
