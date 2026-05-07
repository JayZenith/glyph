# glyph

A message format and training pipeline that teaches LLMs structured, long-horizon task execution. Models emit traces with explicit `plan` / `act` / `response` phases, an internal todo list, and Unicode operators for tagging (`🏷`), referencing (`※`), confidence (`𝑝`), and todo satisfaction (`⊨`).

**Status:** SFT shipped (Qwen3-4B-Base + LoRA). RL via prime-rl is the next stage.

**Artifacts on HF:**
- Model: [`JayZenith/glyph-sft-v1`](https://huggingface.co/JayZenith/glyph-sft-v1) (private)
- Dataset: [`JayZenith/glyph-sft-v1-data`](https://huggingface.co/datasets/JayZenith/glyph-sft-v1-data) (private)

## Format

See [`def.md`](def.md) for the full spec. Example:

```js
plan {
    todo ↦ {
        1 ↦ "Fetch the weather." •
        2 ↦ "Recommend clothing." ※ usr1
    }
}

act {
    call ↦ { tool ↦ get_weather • zip_code ↦ "94103" • id ↦ "w1" } ⊨ 1
}

response「68°F and overcast — light sweater works.」※ "w1" ⊨ 2
```

## Results

**Held-out test loss** (110 traces, never seen in training):

| metric          | base   | sft    | delta        |
|-----------------|--------|--------|--------------|
| mean loss       | 1.280  | 0.972  | **−0.308**   |
| perplexity      | 3.60   | 2.64   | **36% lower** |
| sft beats base  |        |        | **110/110**  |

**Format quality** (5-prompt greedy generation):

| metric                | base | sft   |
|-----------------------|------|-------|
| valid trace           | 0/5  | 4/5   |
| ends with response    | 0%   | 100%  |
| has plan              | 0%   | 100%  |
| no repetition         | 60%  | 100%  |
| not truncated         | 20%  | 100%  |
| used tools when given | 0/4  | 4/4   |
| avg score (out of 7)  | 0.2  | 6.4   |

The single failed valid-trace was a no-tool reasoning prompt where the model wrote a 5-step plan but didn't emit `⊨ N` markers for every step. Clean target for RL.

## Key design decisions

**Why this stack works** — three things had to land together:

1. **`lm_head` in `modules_to_save`.** Default LoRA touches attention + MLP only. Qwen3-Base never saw `<|im_end|>` in pretraining, so without training the lm_head, the model never learns to emit it and never terminates.
2. **Assistant-only loss masking.** Labels are `-100` everywhere except inside `<|im_start|>assistant\n` … `<|im_end|>`. Otherwise the model wastes capacity learning to copy system/user.
3. **Aggressive lm_head LR (2e-5).** First run used 5e-6 for the head — termination still broke. Bumping to match trunk LR fixed it. Single change isolated; the rest of the stack was constant.

LoRA-first ordering also matters: apply LoRA → `enable_input_require_grads()` → `gradient_checkpointing_enable()`. Reverse order disables grads.

## Repo layout

```
train.py                    SFT trainer (ParamGroupTrainer + GenEvalCallback)
rl_train.py                 RL stage (prime-rl, WIP)
validator.py                TASK trace validator (used as RL reward signal)
inference.py                Inference helpers
def.md                      Format spec
docs/                       Design notes
configs/                    Training configs
synthetic_data/             Traces (gitignored)
artifacts/                  Run outputs (gitignored)
scripts/
  eval_sft_formal.py        Generation eval (5 prompts, score format quality)
  eval_test_loss.py         Forward-pass loss on held-out 10% test set
  audit_dataset_diversity.py  Detect template collapse in synthetic data
  patch_dataset.py          Fix recoverable bugs in synthetic traces
  merge_adapter.py          Merge LoRA + lm_head into base for HF upload
  build_bootstrap_dataset.py  Bootstrap initial trace set
  build_continuation_dataset.py  Continue from base policy rollouts
  compare_base_sft.py       Side-by-side base vs SFT generation
  patch_prime_rl_install.py   Patches needed for prime-rl install
  convert_prime_rl_adapter_to_peft.py   Adapter format conversion
```

## Reproduce

### 1. Generate / patch data

Synthesize traces with `generate.py`, then drop in `synthetic_data/`. Patch known bugs:

```bash
python scripts/patch_dataset.py \
    --input synthetic_data/sft_train.jsonl \
    --output synthetic_data/sft_train_1098_official.jsonl
```

Audit for template collapse before training:

```bash
python scripts/audit_dataset_diversity.py --data synthetic_data/sft_train_1098_official.jsonl
```

### 2. SFT

Defaults match the actual run that produced `glyph-sft-v1`. On 1× A100 80GB SXM4 it runs ~1h32m.

```bash
python train.py \
    --model Qwen/Qwen3-4B-Base \
    --data synthetic_data/sft_train_1098_official.jsonl \
    --output runs/sft1 \
    --skip-merge
```

The defaults that produced glyph-sft-v1:
- LoRA rank 64, alpha 64, dropout 0.05
- targets: `q,k,v,o,gate,up,down`
- `modules_to_save=["lm_head"]`
- LR 2e-5 (both trunk and lm_head, separate optimizer groups)
- 3 epochs, effective batch 8 (per-device 1, grad-accum 8)
- max seq length 8192
- 80/10/10 train/val/test split, seed 42
- assistant-only loss masking
- eval every 50 steps incl. greedy gen-eval on 2 held prompts

### 3. Merge LoRA + lm_head into the base

`--skip-merge` above keeps the run light. Merge locally on CPU after pulling the adapter:

```bash
python scripts/merge_adapter.py \
    --base Qwen/Qwen3-4B-Base \
    --adapter runs/sft1/final \
    --output runs/sft1/merged
```

### 4. Eval

**Generation quality** on 5 fixed prompts (validator-scored):

```bash
PYTHONPATH=. python scripts/eval_sft_formal.py \
    --base-model Qwen/Qwen3-4B-Base \
    --sft-model JayZenith/glyph-sft-v1 \
    --output eval_formal.json \
    --max-new-tokens 6000
```

**Held-out test loss** on the 10% held-out tokenized split:

```bash
PYTHONPATH=. python scripts/eval_test_loss.py \
    --base Qwen/Qwen3-4B-Base \
    --sft JayZenith/glyph-sft-v1 \
    --test-set runs/sft1/test_set \
    --output eval_test_loss.json
```

## Validator

`validator.py` enforces TASK structure. Used in two places:
- Eval: scores generations as valid/invalid + per-rule errors
- Future RL: shaped reward signal (validator passes + per-section credit)

Errors (block valid):
- Missing `plan { todo ↦ {...} }` block
- Unsatisfied todos (defined but never marked with `⊨ N`)
- References to undefined tags
- Repetition (≥4 reps of a 20-200 char span)
- Bad termination (output past final `」` other than `<|im_end|>` and known whitespace tokens)
- Call/result `id` mismatch

Run on a dataset:

```bash
python -c "
import json
from validator import validate_trace
n = invalid = 0
for line in open('synthetic_data/sft_train_1098_official.jsonl'):
    n += 1
    if not validate_trace(json.loads(line)['trace']).valid:
        invalid += 1
print(f'{invalid}/{n} invalid')
"
```

## RL plan (next)

- Reward = validator pass + shaped per-section credit (plan present, todos satisfied, ends correctly, tools called when given, no repetition)
- Init from `JayZenith/glyph-sft-v1` (the merged full model, not the LoRA adapter)
- prime-rl harness; install patches in `scripts/patch_prime_rl_install.py`
- Held-out 200-prompt RL set, separate from SFT data and from the 5-prompt format eval
