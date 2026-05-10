# Teaching Qwen3-4B-Base a structured trace format

*Part 1 of a series. SFT is done; RL is next.*

I designed a structured trace format for LLM task execution and SFT'd Qwen3-4B-Base on 1098 synthetic traces. The first run mode-collapsed into infinite repetition and never terminated. This post walks through the diagnosis, the fix, and a 2×2 ablation that isolates which change was load-bearing. The trained model, dataset, and per-prompt eval evidence are public.

- Model: [`JayZenith/glyph-sft-v1`](https://huggingface.co/JayZenith/glyph-sft-v1)
- Dataset: [`JayZenith/glyph-sft-v1-data`](https://huggingface.co/datasets/JayZenith/glyph-sft-v1-data)
- Code: [github.com/JayZenith/glyph](https://github.com/JayZenith/glyph)
- Ablation table + raw eval JSONs: [`docs/ablation.md`](https://github.com/JayZenith/glyph/blob/main/docs/ablation.md)

## The format

Every trace has explicit phases — `plan` (todo list + rationale), `act` (tool calls or thinking), `result` (tool outputs), `response` (final answer to the user) — connected by Unicode operators:

- `🏷` tags an expression for later reference
- `※` references a prior tag
- `⊨ N` marks todo item N as satisfied
- `𝑝 0.0–1.0` annotates confidence

A minimal trace:

```
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

The point is twofold. **For training:** structure makes failure modes legible — a missing `⊨` flags an unfinished todo; a missing tag flags a hallucinated reference. **For RL:** every operator is a deterministic check the validator can score, which means shaped reward without an LM judge in the inner loop.

## The first failure

I generated 1137 synthetic traces with GPT-5-mini playing the role of a stronger policy, kept the 1098 that passed structural validation, and ran a standard LoRA SFT on Qwen3-4B-Base. Rank 64, alpha 64, q/k/v/o + MLP targets, 3 epochs, bf16, gradient checkpointing — nothing exotic.

The model never terminated. Greedy generation ran to the 6000-token cap on every prompt, looping the same phrase ("łazienk" from a Polish trace in the training distribution) for thousands of tokens. Train loss looked fine. Validation loss looked fine. The model had clearly learned *something*, just not the right something.

## Diagnosis

The clue was that the failure was specifically about **terminating**. The model wrote correct plans, called tools correctly, produced response blocks — and then refused to stop. That narrowed the search to one place: the lm_head's prior over `<|im_end|>`.

Qwen3-4B-Base never sees `<|im_end|>` during pretraining. Its initial logit for that token is at the noise floor relative to common continuation tokens. Default LoRA targets attention + MLP only — it does not touch the lm_head. So no matter how many times the SFT data ended with `<|im_end|>`, the gradient could only push the *attention* representation toward predicting it; the lm_head's actual output projection over the vocabulary stayed frozen at its pretraining prior.

The fix:

```python
LoraConfig(
    target_modules=["q_proj", "k_proj", "v_proj", "o_proj",
                    "gate_proj", "up_proj", "down_proj"],
    modules_to_save=["lm_head"],   # full-tensor train, not LoRA-rank constrained
    ...
)
```

`modules_to_save` puts the lm_head in the trainable set as a fully-trained tensor (not LoRA-decomposed). The gradient now flows directly into the output projection that decides which token to emit.

I also added two changes alongside:

1. **Assistant-only loss masking.** Labels are `-100` everywhere except inside `<|im_start|>assistant\n` … `<|im_end|>`, with the trailing `<|im_end|>` *included* in the unmasked span. The model's gradient signal is now concentrated on what it actually has to produce.
2. **A separate optimizer LR for the lm_head.** Custom `Trainer` subclass with two parameter groups — LoRA trunk params at LR `X`, lm_head params at LR `Y`. First run had `X=2e-5, Y=5e-6`; termination still didn't land. Bumping `Y` to `2e-5` (matching trunk) fixed it. This was the only cleanly isolated single-variable change in the original run.

After all three: termination 100%, no repetition, plans written cleanly. Held-out perplexity dropped from 3.60 (base) to 2.64 (SFT). Validator passed 4/5 on a five-prompt smoke eval.

The one failed prompt is informative. The model wrote a 5-step plan but only emitted `⊨ 1` once, not `⊨ 1, ⊨ 2, ..., ⊨ 5`. The validator flagged "Unsatisfied todos: {1, 2, 3, 4, 5}". This is exactly the kind of structural-but-fixable failure RL is for.

## What was load-bearing — the 2×2 ablation

Three changes landed at once. The single isolated ablation (the `Y=5e-6 → 2e-5` bump) only told me about the LR within the full stack. It didn't tell me which of `lm_head`-in-`modules_to_save` or assistant-only masking was load-bearing.

I ran four configs varying both flags. Same data, same seed (42), same hyperparams; the train/val/test split is byte-identical across runs because `train_test_split` is deterministic.

| run | val_loss | test_loss | valid (validator) | ends_with_response | no_repetition | judge_mean | judge_factual |
|---|---|---|---|---|---|---|---|
| A — lm_head + assistant_only | 0.958 | 0.972 | **4/5** | **100%** | **100%** | 3.65 | 3.2 |
| B — none + assistant_only | 0.971 | 0.986 | 0/5 | **0%** | 100% | **3.80** | 3.4 |
| C — lm_head + full_trace | 0.937† | 0.936† | 3/5 | 100% | 100% | 3.55 | 2.8 |
| D — none + full_trace | 0.961† | 0.959† | 0/5 | 60% | **40%** | 3.55 | 3.0 |

† Loss for C/D is averaged over all tokens (full-trace mode); not directly comparable to A/B (assistant tokens only). The clean cross-run signals are the validator and judge columns.

The takeaway is direct: **`lm_head` training is load-bearing.** Removing it (B, D) breaks the model. Keeping it (A, C) keeps it working. Assistant-only masking is a small refinement on top — A beats C by one valid-trace and 0.6 avg score; the format is fine in both, the polish differs.

A bonus finding from D: without lm_head training and without masking, repetition emerges (no_repetition drops from 100% to 40%). The lm_head fix wasn't only suppressing termination failures; it was also keeping the model from getting stuck in token loops.

## Validator vs LM judge — they measure different things

I added an LM judge (`gpt-5-mini`) over the same five generations: `plan_quality`, `response_relevance`, `factual_correctness`, `helpfulness`, each on 1–5.

The interesting result is **B's judge mean is the highest of all four runs (3.80, vs A's 3.65)**. B is structurally unusable — 0% termination, every prompt truncates at 6000 tokens. But the judge reads the trace text and scores what was *written*. The text before truncation is fine: B writes plans, calls tools, even produces something that looks like a response. The judge doesn't penalize the missing `<|im_end|>` because the judge can't see whether the model knew when to stop.

The validator catches that. Same data, two complementary signals:

- **Validator** = usability. Will you get a response back? Does the trace parse?
- **LM judge** = content quality. Conditional on getting *something* back, was it good?

Ship the model when both agree. Both signals say A is the best, but they say it for different reasons.

The other cross-cutting finding: **`judge_factual` is the lowest dim in every run** (2.8–3.4). The judge consistently caught hallucinated specifics — made-up Geekbench scores, invented weather data presented as real. SFT teaches the model to write fluent traces in the right format. It does not teach the model not to invent facts. That's an RL problem.

## What's not done

I want to be precise about what this project does and doesn't show, so the next post doesn't have to walk anything back.

- **Eval is small.** Five prompts, one seed. Headline numbers are real but the variance bars are unmeasured. A 30-prompt × 3-seed pass would put genuine error bars on these metrics. Cheap; not done yet.
- **One model size.** No 1B or 8B comparison. The lm_head story might or might not generalize.
- **One base.** Qwen3-4B-Base specifically. Llama-class bases tie embeddings differently; the same fix may need different plumbing (PEFT auto-detects embed-tied models and unties on merge, which I learned the hard way).
- **Synthesis flags weren't recorded.** I shipped a `data_manifest.json` documenting what we know, but the original `generate.py` invocations weren't checked in. Lesson: every synthetic trace needs a `meta` field with `{teacher_model, prompt_id, sampling_params, timestamp}`. Adding it before regen.
- **Single isolated ablation in the original run** (lm_head LR), plus the 2×2 ablation here. That's two cleanly attributed effects. Real research has dozens.

These are honest gaps. Some I'll close before RL (the meta field, the 30-prompt eval). Some are scope I'm explicitly punting (multi-base, multi-size).

## Why bother with the structured format at all

Two reasons that compound.

1. **Reward shaping is mechanical.** Every operator (`🏷`, `※`, `⊨`, response termination, call/result id pairing) is a deterministic check. Writing the reward function is a couple hundred lines of regex, not an LM-as-a-judge inner loop. The LM judge augments — it scores semantic quality after the structural reward — but the bulk of the gradient signal comes from cheap, fast, deterministic checks.
2. **Failures are legible.** When the model regresses, I know whether it's a "missed satisfaction marker" failure or a "hallucinated tag reference" failure or a "termination" failure. Each is a different fix. Compare to a free-form chat model where every regression looks like "it got dumber".

Both reasons matter more for RL than for SFT.

## Next: RL

The plan, in order:

1. Fix the dataset-provenance gap (meta field + checked-in build script).
2. Expand the format eval to ~30 prompts, 3 seeds. Get variance bars.
3. Define the shaped reward: `α·validator_pass + β·per_section_credit + γ·judge_score − δ·KL(policy ‖ SFT)`. Tune `γ` and the hallucination penalty against the `judge_factual` signal already collected.
4. RL via prime-rl, init from `JayZenith/glyph-sft-v1`. Held-out 200-prompt RL set, separate from both the SFT data and the format eval.
5. Watch for reward hacking — minimal traces that satisfy the validator but say nothing useful are the obvious failure mode. KL penalty + hold-out judge prompts as the guardrails.

The headline I want from RL is "validator pass + judge factual both go up, with KL bounded". If that lands, the format-as-RL-substrate thesis works.

If it doesn't land, I want to know precisely why — which is the whole point of building the eval rigorously first.

---

*Code, model, dataset, ablation evidence: [github.com/JayZenith/glyph](https://github.com/JayZenith/glyph). Comments and corrections welcome.*
