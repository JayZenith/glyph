# SFT to RLVR for a Rust tool-use agent: the win disappeared when the eval got stricter

I fine-tuned `Qwen3-4B-Base` into a small Rust tool-use agent, then tried to improve it with RLVR. The original goal was clean and reasonable:

1. build a real SFT artifact,
2. evaluate it against held-out Rust tool-use tasks,
3. use verifier-based RL to improve the failures.

The short version is less satisfying but more useful: **SFT worked; the RLVR win did not survive audit.**

SFT_V1 learned the tool protocol and solved a meaningful chunk of the held-out eval. The RL runs after it either regressed the model, moved a narrow metric that later turned out to be scored too loosely, or were basically flat after strict cargo-verifier + clean-`FINAL` scoring. The project is now best understood as an SFT/eval artifact plus a negative RLVR result, not as a successful RLVR model.

That is frustrating, but it is also the real result.

---

## The agent

The model speaks a tiny tool protocol:

```text
assistant -> CALL read_file(id="c1", file_path=".../src/lib.rs")
tool      -> RESULT c1: status: success stdout: ...
assistant -> CALL apply_patch(id="c2", ...)
tool      -> RESULT c2: status: success stdout: patch applied
assistant -> CALL cargo_test(id="c3", project_path="...")
tool      -> RESULT c3: status: success stdout: ...
assistant -> FINAL: fixed the branch logic and verified tests pass.
```

The tools execute for real against sandboxed Rust crates. `cargo_test` only succeeds if tests pass. `cargo_run` only succeeds if stdout matches the oracle. There is no reward model here; the verifier is the compiler/tests/runtime output.

The main SFT model is:

```text
JayZenith/SFT_V1
```

It was trained from `Qwen3-4B-Base` on synthetic-but-executed traces: GPT-authored specs were materialized into real crates, the planned tool trajectories were executed locally, and only executable traces were kept.

---

## What SFT_V1 actually achieved

The original held-out 69 eval showed SFT_V1 was useful, but the headline metric needed auditing.

The old summary emphasized `terminal_tool_success`, but that metric was too loose: it could count a final successful non-verifier tool, such as `read_file`, instead of an actual successful `cargo_test` or `cargo_run`.

The stricter interpretation is:

```text
valid trace = cargo_test/cargo_run success + clean FINAL
```

Under that stricter view, SFT_V1's real held-out-69 result was:

```text
SFT_V1 valid traces / actual cargo successes: 52/69
```

That is the SFT artifact. It is not perfect, but it is real: a 4B base model became a working Rust tool-use model on a held-out set.

The persistent failure mode was not just "the model cannot use tools." It could use tools. The hard cases mixed several things:

- actual Rust/task failures,
- long multi-turn traces,
- repeated patch/test loops,
- missed or dirty `FINAL`,
- sensitivity to inference harness and sampling.

---

## The first RLVR idea: fix stopping

The first RLVR attempts tried to fix the apparent stopping gap: cases where the model seemed to reach a successful verifier result but failed to emit a clean `FINAL`.

That idea did not work.

`RLVR_V1` regressed badly. A later corrected-reward attempt also regressed. The important diagnostic was that the RL training prompts did not reproduce the same churn/finalization failure mode that appeared in the held-out eval. On the training distribution, SFT_V1 showed essentially no post-success churn, so the RL reward had little useful contrast for "stop after success."

The lesson was simple:

```text
RL cannot reinforce a behavior distinction that does not appear in its rollout distribution.
```

If the training rollouts do not contain the failure mode, the verifier reward cannot fix it.

---

## The second RLVR idea: lift partial solve-rate

The next idea was more aligned with GRPO.

GRPO only learns from prompts where rollouts in the same group get different rewards. If all rollouts pass, advantage is zero. If all rollouts fail, advantage is zero. The useful band is:

```text
0 < pass@k < k
```

So we scanned SFT_V1 on 134 depth>=3 train prompts:

```text
95 solved at 8/8
39 mixed
0 true capability gaps
```

That already predicted trouble. Most of the dataset was saturated. The mixed prompts were mostly near-solved, often 7/8. That is not a rich capability frontier; it is mostly sampling instability.

We still ran GRPO on the 39 mixed prompts.

The matched vLLM-vs-vLLM result was:

```text
SFT_V1:  283/312 = 0.907
step_25: 273/312 = 0.875
step_50: 272/312 = 0.872
```

Prompt movement at step 25:

```text
6 up
13 down
20 flat
```

So the capability-lift run did not lift. It sharpened a few prompts and damaged more. SFT_V1 remained the better model on that set.

---

## The scoring bug that changed the story

At this point the project looked like it had a narrow held-out-failure pass@8 win. That was premature.

The pass@k scanner and formal eval had to be audited. The issue was `terminal_tool_success`: it was not strict enough. It could mark traces as terminal-successful even when the last successful tool was not a cargo verifier.

The scanner was fixed to track:

```text
cargo_verifier_success
valid_trace
```

Where:

```text
cargo_verifier_success = successful cargo_test/cargo_run
valid_trace = cargo verifier success + clean FINAL
```

After this correction, the original 17 formal SFT failures were rescanned with vLLM pass@8. The corrected result was:

```text
17 original formal failures
11 valid-mixed prompts
5 true 0/8 valid/cargo gaps
1 cargo 1/8 but valid 0/8, excluded from strict target set
```

That produced a stricter 11-prompt RL target set:

```text
synthetic_data/rl_prompts_heldout69_fail17_valid_mixed11.jsonl
```

---

## The LoRA retry

Because full-parameter GRPO had repeatedly damaged the SFT model, we restored a LoRA path in PRIME-RL and tried the stricter 11-prompt target set.

The goal was modest: reduce blast radius and see whether strict valid-trace pass@8 improved.

The baseline on those 11 prompts was:

```text
SFT_V1: 31/88 valid traces
```

The LoRA step-25 result was:

```text
LoRA step_25: 32/88 valid traces
delta: +1
prompts: 4 up, 5 down, 2 flat
```

Prompt-level movement:

```text
+4  eval100_022  1/8 -> 5/8
 0  eval100_044  1/8 -> 1/8
-2  eval100_025  3/8 -> 1/8
+1  eval100_014  1/8 -> 2/8
 0  eval100_039  2/8 -> 2/8
+1  eval100_047  6/8 -> 7/8
-1  eval100_036  5/8 -> 4/8
-1  eval100_057  5/8 -> 4/8
-1  eval100_064  1/8 -> 0/8
+1  eval100_005  3/8 -> 4/8
-1  eval100_037  3/8 -> 2/8
```

That is not a win. It is basically flat, with prompt-level instability.

The LoRA merge was verified as real: all adapter tensors loaded and probe weights changed. The flat result was not a merge no-op.

Artifacts:

```text
results/RLVR_LORA_HELDOUT69_VALID_MIXED11/step25_valid_mixed11_passk8.json
results/RLVR_LORA_HELDOUT69_VALID_MIXED11/step25_valid_mixed11_passk8.log
outputs/rlvr_lora_heldout69_mixed11_bs16/run_default/broadcasts/step_25/
```

---

## The latest attempted expansion

After the 11-prompt LoRA run came back flat, we tried to build a larger target pool similar to the held-out failures.

We generated 150 heldout-failure-like specs and materialized them into executable crates. The materialization process is strict: the generated spec must execute exactly as planned, including intentional failed verifier steps for recovery traces and final verifier success.

Accepted:

```text
68/150
```

Reject breakdown:

```text
expected final success failed: 52
expected fail passed early:    13
invalid JSON:                  10
bad patch find count:           7
```

The accepted set is:

```text
synthetic_data/heldout_fail_like_plus150_accepted.jsonl
sft/evals/heldout_fail_like_plus150.yaml
```

We started a corrected SFT_V1 pass@4 screen on those 68 prompts:

```text
model: JayZenith/SFT_V1
k: 4
temperature: 0.8
metric: valid_trace = cargo success + clean FINAL
```

As of the current run snapshot:

```text
19/68 done
5 mixed rlvr-targets
2 solved
12 capability gaps
```

Projected from that early rate, this is likely to yield around 18 mixed prompts. That is probably still too small for a serious RLVR run. The correct cutoff should have been enforced earlier:

```text
<25 mixed prompts: stop
25-40: fragile LoRA-only experiment
40+: maybe worth one serious LoRA run
```

This latest screen is therefore an audit, not a commitment to another RL run.

---

## Why the RLVR attempts failed

The failures now look less mysterious.

First, the original stopping RL did not train on the actual stopping failure. The failure was visible in one eval harness, but not in the RL rollout distribution.

Second, the broad 39-prompt pass@k run targeted a near-ceiling band. Most prompts were already solved or almost solved. There was little stable capability gradient and plenty of room for drift.

Third, the strict held-out mixed set had only 11 prompts. That is too small. LoRA reduced damage, but it did not create a reliable lift.

Fourth, sparse verifier reward does not distinguish robust solutions from lucky ones unless the data and reward are designed to expose that distinction. A rollout that passes once can get reinforced even if the underlying strategy is brittle.

Fifth, the earlier "narrow win" depended on a metric that was not strict enough. Once the scanner required cargo verifier success plus clean `FINAL`, the result became flat.

---

## What is still valuable

The SFT artifact is valuable.

The corrected eval stack is valuable.

The negative RLVR result is valuable if stated honestly.

The project now supports these claims:

- SFT can teach a 4B base model a real Rust tool-use protocol.
- Held-out evals must distinguish terminal tool status from actual cargo verifier success.
- `pass@k` is useful for finding latent capability, but only if the success metric is strict.
- GRPO needs mixed reward groups; saturated and all-fail prompts provide no useful gradient.
- Tiny RLVR target sets are unstable, even with LoRA.
- Full-parameter RLVR can damage the SFT protocol prior.
- A claimed RLVR win must survive matched before/after evaluation with strict cargo+FINAL scoring and preferably a held-out mixed split.

The project does **not** currently support this claim:

```text
RLVR produced a better deployable Rust tool-use model than SFT_V1.
```

It did not.

---

## Reproducible commands

Generate the larger heldout-failure-like screening batch:

```bash
python3 synthetic_data/batch_specs.py build-heldout-failure-like \
  --count 150 \
  --custom-prefix heldoutfailplus150 \
  --output synthetic_data/batch_heldout_fail_like_plus150/requests.jsonl

python3 synthetic_data/batch_specs.py submit \
  --input synthetic_data/batch_heldout_fail_like_plus150/requests.jsonl \
  --metadata synthetic_data/batch_heldout_fail_like_plus150/batch.json \
  --task-name glyph_heldout_failure_like_plus150_specs
```

Retrieve and materialize:

```bash
python3 synthetic_data/batch_specs.py retrieve \
  --metadata synthetic_data/batch_heldout_fail_like_plus150/batch.json \
  --output synthetic_data/batch_heldout_fail_like_plus150/results.jsonl

python3 synthetic_data/materialize_specs.py \
  synthetic_data/batch_heldout_fail_like_plus150/results.jsonl \
  --source-root synthetic_data/blueprints \
  --cases-root runs/materialize_heldout_fail_like_plus150_cases \
  --families-dir synthetic_data/batch_heldout_fail_like_plus150/families \
  --rejects synthetic_data/batch_heldout_fail_like_plus150/rejects.jsonl \
  --tool-timeout 30
```

Build the eval YAML:

```bash
python3 synthetic_data/build_eval_prompts.py \
  synthetic_data/heldout_fail_like_plus150_accepted.jsonl \
  --source-root synthetic_data/blueprints \
  --output sft/evals/heldout_fail_like_plus150.yaml \
  --section heldout_fail_like_plus150 \
  --trace-root runs/heldout_fail_like_plus150_cases \
  --seed 2026
```

Screen SFT_V1 with corrected vLLM pass@4:

```bash
PYTHONPATH=. python sft/passk_scan_vllm.py \
  --sft-model JayZenith/SFT_V1 \
  --prompt-file sft/evals/heldout_fail_like_plus150.yaml \
  --prompt-section heldout_fail_like_plus150 \
  --cases-root runs/heldout_fail_like_plus150_cases \
  -k 4 \
  --temperature 0.8 \
  --max-new-tokens 4000 \
  --max-tool-rounds 15 \
  --tool-timeout 30 \
  --output results/SFT_V1_HELDOUT_FAIL_LIKE_PLUS150_PASSK4/sft_v1_passk4_68.json \
  --no-resume \
  --gpu-memory-utilization 0.90 \
  --max-model-len 20000 \
  --dtype bfloat16 \
  --save-rollouts
```

Strict LoRA heldout-failure mixed-set eval:

```bash
PYTHONPATH=. python sft/passk_scan_vllm.py \
  --sft-model outputs/rlvr_lora_heldout69_mixed11_bs16/merged_step_25 \
  --names "$(paste -sd, synthetic_data/heldout69_fail17_valid_mixed11_names.txt)" \
  -k 8 \
  --temperature 0.8 \
  --max-new-tokens 4000 \
  --max-tool-rounds 15 \
  --tool-timeout 30 \
  --output results/RLVR_LORA_HELDOUT69_VALID_MIXED11/step25_valid_mixed11_passk8.json \
  --no-resume \
  --gpu-memory-utilization 0.86 \
  --max-model-len 12288 \
  --dtype bfloat16 \
  --save-rollouts
```

---

## Current conclusion

The clean artifact is SFT_V1 plus corrected evals.

The RLVR artifact is negative:

```text
full fine-tune RLVR: regressed
39-prompt pass@k RLVR: regressed
11-prompt strict LoRA RLVR: 31/88 -> 32/88, effectively flat
```

The honest next step is not another rushed RL run. It is to finish the SFT/eval story, preserve the failed RLVR runs as evidence, and only revisit RLVR if there is a genuinely large mixed set with a held-out split.

