# SFT Worked. RLVR Showed One Hard Signal, Then Taught Me Where the Harness Was Lying.

This is where I am ending this round of Glyph.

Glyph is a Rust tool-use agent. The model is not just answering coding
questions. It emits `CALL tool(...)` blocks, the tools run against real Rust
crates, and then the model is supposed to stop with a clean `FINAL`.

The target trace is simple:

```text
CALL read_file(...)
RESULT ...
CALL apply_patch(...)
RESULT ...
CALL cargo_test(...) or CALL cargo_run(...)
RESULT status: success
FINAL: ...
```

The whole experiment was: can I take a strong SFT model and use verifier RL to
make it better on a held-out Rust tool-use eval?

The final answer is not the clean RLVR result I wanted.

It is this:

```text
SFT built the agent.
RLVR produced one real held-out signal: a strict solve that SFT missed.
RLVR did not improve aggregate held-out reliability.
The most valuable part of RLVR was exposing every mismatch in the training and
eval plumbing.
```

That sounds less glamorous than "RL solved coding." It is also the result I
trust.

## The Metric Had To Get Strict

Early on, I had a metric called `terminal_tool_success`.

That was too loose. It could give credit to traces where the last successful
tool was something like `read_file`. That is not a coding-agent success. The
agent needs to make the crate pass.

The metric that mattered became:

```text
valid_trace =
  terminal cargo_test or cargo_run success
  + clean FINAL after that verifier success
  + exact CALL syntax
  + no role-marker leakage
  + no repetition or gibberish
  + no extra tool use after the successful verifier
```

This is picky on purpose. If the model passes tests but then keeps patching, or
emits malformed `CALL` syntax, or never finalizes, it is not a usable tool-use
agent.

That metric shaped the rest of the work.

## SFT Was the Real Breakthrough

The first strong SFT model, `SFT_V1`, scored:

```text
SFT_V1 strict held-out-69: 52/69
```

That was the first real result. A 4B base model learned the
CALL/RESULT/FINAL protocol well enough to edit Rust projects, run real cargo
verifiers, and solve most of the held-out set.

Then I tried deeper SFT data.

`SFT_V2` used `signal_v2_1323` with more variable-depth recovery. `SFT_V3` used
`signal_v3` with deeper recovery and oversampled clean PASS -> FINAL traces.

The broad held-out scores were:

![Strict held-out-69 scores for SFT models](assets/final_sft_scores.svg)

```text
SFT_V1:     52/69
SFT_V2:     48/69
SFT_V3:     50/69
SFT_HALF_A: 51/69
```

But the hard-tail story was different. On the original 17 held-out problems
that `SFT_V1` failed:

```text
SFT_V1: 0/17
SFT_V2: 4/17
SFT_V3: 5/17
```

That was the first important tradeoff. Deeper recovery data gave the model more
hard-tail capability, but it cost broad reliability. More difficult traces were
not just "more data." They shifted the policy.

## The Clean Split

To make the RLVR experiment fair, I split `synthetic_data/signal_v3.jsonl` into
two deterministic halves:

```text
SFT_HALF_A: 1,042 rows, 762 unique case_ids
RL_POOL_B:  1,041 rows, 760 unique case_ids
```

The split preserved family, difficulty, expected tool-sequence length, and
run-vs-test balance as much as possible. It was grouped by `case_id`, so
oversampled traces for the same case could not land on both sides.

The leakage checks were clean:

```text
case_id overlap: 0
trace overlap:   0
```

I trained `SFT_HALF_A` only on half A. It got:

```text
SFT_HALF_A strict held-out-69: 51/69
```

That was close enough to `SFT_V1` to be a good baseline. The plan was then to
run LoRA RLVR from `SFT_HALF_A` on `RL_POOL_B` and ask one question:

```text
Does RLVR exceed 51/69 on the same strict held-out eval?
```

## The RLVR Attempts Mostly Failed

The first RLVR attempts were not clean.

Full-finetune RLVR was too destructive. It damaged the SFT prior.

A tiny target set was not enough. The strict 11-prompt LoRA run was basically
flat: `31/88 -> 32/88`.

Then I moved to the cleaner `RL_POOL_B` setup. That exposed the real failure
modes.

## Failure Mode 1: Reward Was Easier Than Eval

The RL reward initially gave too much credit for "cargo passed somewhere in the
trace." The held-out eval wanted cargo success and a clean final stop.

That mismatch matters. If the reward says "good enough" before the eval would
say "valid trace," RL optimizes the wrong behavior.

The reward was patched so the top score matched held-out success:

```text
top reward = held-out-style valid trace
```

Partial rewards for verifier success without a clean final were removed or
clamped so they could not become the main optimization target. Protocol errors,
malformed calls, role leakage, gibberish, repetition, and extra tool use after a
passing verifier block the top reward.

That did not make RLVR better, but it removed one source of fake progress.

## Failure Mode 2: The Model Format Had To Be Exact

The model was trained to see literal ChatML-style turns:

```text
<|im_start|>assistant
CALL ...
<|im_end|>
<|im_start|>tool
RESULT ...
<|im_end|>
```

Any deviation changes the task. During the RL runs, I had to inspect W&B
samples and raw trajectories to make sure the model was seeing the same
protocol during RL that it saw during SFT and eval.

This is one of the most annoying lessons of tool-use training: if your protocol
is part of the learned behavior, the protocol is not boilerplate. It is the
task.

## Failure Mode 3: Bad Exports Looked Like Bad Models

This was the nastiest bug.

Two RLVR checkpoints appeared to collapse to `0/69`. The traces were not random
garbage. They all looked like this:

```text
CALL read_file(id="c1", file_path="..."))
```

One extra `)`.

The strict parser rejected every malformed call, no tools ran, no final answer
was emitted, and the eval went to zero.

At first glance, that looks like RL destroyed the model. It did not.

The bad `0/69` models came from non-canonical full-weight export paths. The
policy served during RL was base model plus the broadcast LoRA adapter. The
trainer's `weights/step_N` directory was not the same thing as a clean
serveable Hugging Face model. Assembling or merging the wrong artifact produced
a model that was close enough to look plausible but different enough to flip a
low-margin token after the call line.

The fix was to make the official artifact the PEFT adapter exported from:

```text
outputs/<RUN>/run_default/broadcasts/step_N/
```

and evaluate it as:

```bash
python -m sft.eval_formal \
  --sft-model JayZenith/SFT_HALF_A \
  --sft-adapter JayZenith/<RLVR_ADAPTER_REPO> \
  ...
```

That removed the export-collapse story from the final analysis.

## The One Real RLVR Signal

The cleanest RLVR signal is `RLVR_V1000` step 25, evaluated through a direct
local merge of the broadcast adapter.

The result:

```text
SFT_HALF_A:                         51/69
RLVR_V1000 step25 direct merge:     50/69
```

So it did not improve overall.

But it did solve one held-out problem that `SFT_HALF_A` failed:

![V1000 case-level diff](assets/final_v1000_case_diff.svg)

The case-level signal was:

```text
eval100_039_select_event_codes_partial_then_full_fix
kind: patch_test_recover
```

`SFT_HALF_A` failed with:

```text
missing_final
dirty_final
final_before_tool_completion
task_failure
```

The SFT model got stuck in a repeated read -> patch -> test loop. It made 21
tool calls, never got the tests to pass, exhausted the budget, and emitted no
clean final.

`RLVR_V1000` solved it. It made a better fourth patch, got `cargo_test` passing
at call `c12`, and emitted a clean one-line `FINAL`.

That is a real held-out solve. It is not an export artifact. It is not lenient
scoring. It passed the strict eval. It is still only a signal, because the same
checkpoint regressed two other cases.

But two cases flipped the other way:

```text
eval100_048_dispatch_action_match_branch_repair
eval100_085_log_window_filter_map_recover
```

Both were solved by `SFT_HALF_A` and failed by `RLVR_V1000`. These were not
finalization-only regressions. They were patching regressions: the RLVR policy
followed a different greedy recovery path and failed to converge.

That is the honest claim:

```text
RLVR showed one case-level recovery signal absent from SFT greedy behavior,
while aggregate reliability regressed by one.
```

I count that as evidence RLVR can move useful recovery behavior into the greedy
policy. I do not count it as a win.

## The Final Clean RLVR Run Still Did Not Improve

After fixing reward shape, protocol format, strict parser behavior, and adapter
export, I ran the cleaner V3000-style LoRA path.

The evaluated checkpoints were:

![RLVR held-out-69 did not beat SFT_HALF_A](assets/final_rlvr_scores.svg)

```text
SFT_HALF_A:            51/69
V3000 step 5 adapter:  44/69
V3000 step 10 adapter: 47/69
V3000 step 15 adapter: 43/69
```

Step 10 was better than step 5 and step 15, but still below the baseline. The
run was not an improvement.

The important part is that this time the failure was not obviously a parser
bug, reward hack, or bad export. It looked more like real policy drift and
weak/noisy learning. The RL pool was hard, recovery-heavy, and not filtered to
examples where the base model had mixed pass/fail outcomes. A lot of the
gradient was probably coming from trajectories that were either hopeless or
unstable rather than from clean near-boundary learning signal.

## What I Think Happened

SFT gave the model a strong prior for the exact tool protocol and a decent
greedy repair strategy.

RLVR was able to perturb that policy enough to change recovery trajectories.
Sometimes that helped. The V1000 case-level signal is the proof.

But the same perturbation also broke other recoveries. The model was not
learning a broadly better repair policy. It was reshuffling near-boundary
greedy paths.

The best interpretation is:

```text
RLVR found a real capability movement, not a reliable improvement.
```

That distinction matters.

If I wanted to keep going, I would not just train longer. I would change the
data selection:

```text
1. Run pass@k on RL_POOL_B with the SFT model.
2. Keep prompts with mixed outcomes under the same eval budget.
3. Drop always-solved and always-failed prompts.
4. Train with frequent greedy canaries.
5. Stop at the first checkpoint that improves held-out valid_trace.
```

The next experiment should be about signal quality, not more compute.

## Where This Ends

The result I am taking away:

```text
SFT is the main result.
Strict evals are non-negotiable.
RLVR can create a held-out recovery signal.
This RLVR setup did not improve aggregate reliability.
Most "RL collapse" was actually harness, reward, or export mismatch until those
were audited away.
```

That is enough for this run.

The project now has a clean split, a strict eval, adapter-based RLVR export, and
a reproducible record of what worked and what did not.

I wanted a simple RLVR result. The actual finding is more useful:

```text
Verifier RL is only as real as the contract it is optimizing.
For tool-use agents, the contract is the whole trace, not just the verifier.
```
