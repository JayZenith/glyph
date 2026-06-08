# SFT worked. RLVR found every mismatch in the harness.

This project started with a simple question:

```text
Can verifier RL improve a small Rust tool-use agent after SFT?
```

The agent is a `Qwen3-4B-Base` model fine-tuned to edit and verify real Rust
crates. It emits tool calls, the tools execute for real, and then the model is
supposed to stop with one final answer.

The clean target behavior looks like this:

```text
<|im_start|>assistant
CALL read_file(id="c1", file_path=".../src/lib.rs")
<|im_end|>

<|im_start|>tool
RESULT c1:
status: success
stdout:
...
<|im_end|>

<|im_start|>assistant
CALL apply_patch(id="c2", file_path=".../src/lib.rs", find="...", replace="...")
<|im_end|>

<|im_start|>tool
RESULT c2:
status: success
stdout:
patch applied
<|im_end|>

<|im_start|>assistant
CALL cargo_test(id="c3", project_path="...")
<|im_end|>

<|im_start|>tool
RESULT c3:
status: success
stdout:
running 3 tests
...
<|im_end|>

<|im_start|>assistant
FINAL: Fixed the logic and verified the tests pass.
<|im_end|>
```

The important part is that `cargo_test` and `cargo_run` are not simulated
rewards. They run against sandboxed Rust crates. A pass means the code compiled
and the verifier accepted it.

The first version of the story would have been easy to tell: SFT made the model
use tools, RLVR improved it, ship the result. That is not what happened.

The real story is more useful:

```text
SFT produced a real Rust tool-use model.
RLVR repeatedly regressed it.
Each regression exposed a hidden mismatch between training and evaluation.
```

This writeup is the audit trail so far.

---

## The metric that matters

At first, we looked at a loose metric called `terminal_tool_success`.

That was wrong.

It could count a successful terminal non-verifier tool, such as `read_file`, as
success. For a coding agent, that is not the task. The model has to make the
crate pass.

The strict metric became:

```text
valid_trace =
  successful cargo_test or cargo_run
  + exactly one clean FINAL after that successful verifier
  + no malformed CALL syntax
  + no role-marker leakage
  + no repetition or gibberish
  + no extra tool use after finalization
```

That sounds picky, but it is the actual deployment contract. A tool-use model
that passes tests but emits malformed calls, leaks chat markers, or never
finalizes is not reliably usable.

---

## SFT_V1: the first real artifact

The first successful SFT model was `SFT_V1`.

On the held-out 69-problem eval, strict scoring gave:

```text
SFT_V1: 52/69 valid traces
```

That is the first important result. A 4B base model learned a real Rust
CALL/RESULT/FINAL protocol and solved most of a held-out tool-use eval.

The failures were not just "the model cannot call tools." The model could read
files, patch code, and run cargo. The hard cases were a mixture of:

- longer recovery traces,
- failed-test repair loops,
- run-vs-test verifier differences,
- missing or dirty final responses,
- protocol sensitivity under different inference harnesses.

SFT worked, but it left a hard tail.

---

## SFT_V2 and SFT_V3: hard-tail capability vs broad reliability

The next SFT datasets added deeper recovery traces.

The hope was that more variable-depth recovery data would help the model solve
the difficult cases that SFT_V1 missed.

The held-out-69 results were:

```text
SFT_V1: 52/69
SFT_V2: 48/69
SFT_V3: 50/69
```

So the broad held-out score did not improve.

But on the original 17 problems SFT_V1 failed:

```text
SFT_V1: 0/17
SFT_V2: 4/17
SFT_V3: 5/17
```

That was a useful clue. The deeper SFT data did add hard-tail capability, but it
also disturbed broad reliability. More difficult traces were not simply better;
they shifted the model.

This pushed the project toward a cleaner SFT -> RLVR experiment:

1. Train SFT on one half of the best synthetic dataset.
2. Keep the other half for RLVR.
3. Evaluate both SFT and RLVR on the same held-out 69.
4. Judge only strict `valid_trace`.

---

## The clean split

The dataset `synthetic_data/signal_v3.jsonl` was split deterministically:

```text
SFT_HALF_A: 1042 rows, 762 unique case_ids
RL_POOL_B:  1041 rows, 760 unique case_ids
```

The split was stratified by:

- family,
- difficulty,
- expected tool-sequence length,
- run vs test verifier family.

It was also grouped by `case_id`, so duplicate traces for the same case could
not leak across SFT and RL.

The leakage checks were clean:

```text
case_id overlap: 0
trace overlap:   0
```

The RL prompt set derived from pool B had:

```text
RL_POOL_B_PROMPTS: 760 prompts
```

This was the right setup in principle. The SFT and RL training sets did not
overlap, and RLVR had many more prompts than the earlier 11- or 39-prompt
experiments.

The SFT model trained on half A scored:

```text
SFT_HALF_A: 51/69 valid traces
```

That was close enough to SFT_V1 to be a fair baseline.

---

## Early RLVR attempts: too small, too destructive, or flat

Before the clean split, several RLVR attempts had already failed.

Full-finetune RLVR regressed badly. It damaged the SFT protocol prior.

A 39-prompt pass@k target set also regressed. The set was too narrow and mostly
near-saturated. GRPO needs within-group reward contrast; prompts that are always
solved or always failed do not provide useful advantage.

A stricter LoRA run on 11 held-out mixed prompts was almost flat:

```text
SFT_V1 baseline: 31/88 valid traces
LoRA step 25:    32/88 valid traces
```

That was not a meaningful win.

The lesson at that stage was:

```text
Tiny target sets are unstable.
Full finetuning is too destructive.
LoRA reduces blast radius, but it does not create signal from nothing.
```

That is why the clean SFT_HALF_A / RL_POOL_B experiment mattered.

---

## RL_POOL_B: the first large clean run still collapsed

The first LoRA RLVR run from `SFT_HALF_A` trained on `RL_POOL_B`.

The high-level command shape was:

```text
base model:    JayZenith/SFT_HALF_A
teacher/KL:    JayZenith/SFT_HALF_A
data:          synthetic_data/rl_prompts_signal_v3_pool_b.jsonl
LoRA:          rank 16, alpha 32
rollouts:      8 per example
batch size:    48
max tokens:    4000 completion tokens
zero-advantage filtering: on
```

This should have had a fair chance. It was LoRA, it used the SFT model as the
teacher anchor, and the RL pool was the held-out half of the synthetic training
distribution.

But the held-out result was terrible:

```text
SFT_HALF_A:              51/69
RL_POOL_B step 25 V2:    15/69
```

That was not noise. It was a collapse.

The failure buckets showed the model had stopped behaving like the eval model:

```text
task_failure:                 54
missing_final:                53
dirty_final:                  53
final_before_tool_completion: 53
truncated:                     5
```

The RL run had optimized something, but not the held-out behavior.

---

## The first hidden mismatch: reward shape

The original RL reward gave partial positive credit for verifier success even
without a clean final answer.

That sounds reasonable until you remember the eval:

```text
cargo success without clean FINAL is not a valid trace
```

If RL gets positive reward for "cargo passed, but the transcript is invalid",
then the optimizer is allowed to move probability mass toward outputs that the
held-out eval rejects.

So the reward was changed:

```text
only exact heldout-style success is positive
```

Cargo success without clean `FINAL` can be less bad than total failure, but it
must not be a positive optimization target.

That change was locked with golden tests. The reward now requires:

- successful terminal `cargo_test` or `cargo_run`,
- no later tools after the passing verifier,
- exactly one clean `FINAL`,
- no malformed CALL syntax,
- no bad cargo project paths,
- no role marker leakage,
- no gibberish or repetition.

This aligned the top reward with the held-out metric.

---

## The second hidden mismatch: the model was not seeing the same chat format

The SFT model was trained on a specific ChatML-style protocol:

```text
<|im_start|>assistant
CALL ...
<|im_end|>

<|im_start|>tool
RESULT ...
<|im_end|>

<|im_start|>assistant
...
```

During RL debugging, W&B samples exposed a dangerous ambiguity: some internal
views showed role strings that looked different from the actual tokens the model
should see.

For this kind of model, that matters. The model is not learning an abstract API;
it is learning an exact token protocol. If RL renders turns differently from SFT
or eval, the policy can drift immediately.

The RL code was patched to force the Glyph chat template and to keep the model's
training-time structure aligned with eval.

This fixed one class of mismatch, but not the whole problem.

---

## The third hidden mismatch: malformed CALLs worked during RL

The next collapse was the most revealing.

After tightening reward and chat formatting, an RL checkpoint evaluated at:

```text
RLVR_V888 step 25: 0/69 valid traces
```

The key summary was:

```text
exact_call_syntax_rate: 0.0
terminal_tool_success_rate: 0.0
valid_traces: 0
```

Every held-out trace failed strict syntax.

The underlying issue was that RL execution was more permissive than eval. During
RL, malformed calls could still be parsed well enough to execute tools. During
eval, exact syntax was required.

A representative bad pattern is:

```text
CALL read_file(id="c1", file_path="src/lib.rs"))<|im_end|>
```

That has an extra `)`. A permissive extractor can still see something that looks
like a `read_file` call and execute it. The strict eval rejects it as malformed
CALL syntax.

So RL had discovered a local optimum:

```text
emit sloppy tool calls that the RL environment still executes,
get useful tool feedback,
but fail the real evaluator.
```

This is why greedy held-out decoding looked so bad. Greedy decoding removed the
sampling noise and showed the actual policy mode after RL: the highest
probability behavior had shifted toward malformed-but-executable calls.

That is not a model mystery. It is reward hacking against a permissive
environment.

---

## The fixes now in place

The current code has been tightened around the real contract.

The protocol parser now rejects malformed CALL lines that eval rejects. Examples
like extra parentheses, bad separators, duplicate arguments, missing ids, bad
terminators, and generated special-token tails cannot become high-reward tool
trajectories.

The RL reward now uses exact held-out-style success as the only positive reward.

The RL chat rendering is forced to the Glyph protocol the SFT model was trained
on.

The canary path exists so a run can be stopped early if greedy eval behavior
collapses.

The reward golden tests encode the key invariants:

- exact solve + clean final gets the top reward,
- cargo success without clean final is not positive,
- malformed calls score poorly,
- bad cargo project paths block top reward,
- garbage after final is not clean success,
- failed verifier retries are bounded,
- no-call outputs are discouraged.

These changes do not prove RLVR will now work. They only remove known invalid
training signals.

---

## Current scoreboard

Strict held-out-69 `valid_trace`:

```text
SFT_V1:                 52/69
SFT_V2:                 48/69
SFT_V3:                 50/69
SFT_HALF_A:             51/69
RL_POOL_B step 25 V2:   15/69
RLVR_V888 step 25:       0/69
```

The 0/69 result is not because the base model forgot Rust from one LoRA step in
some mysterious way. It is because the RL environment allowed behavior the eval
forbids, and the optimizer found it.

That is the central lesson of the project so far.

---

## What this project can honestly claim

The project supports these claims:

- SFT can teach a 4B base model a real Rust tool-use protocol.
- Strict eval must require verifier success, not generic terminal tool success.
- Harder SFT traces can improve hard-tail cases while lowering broad reliability.
- SFT/RL data splits need case-level deduplication, not just row-level shuffling.
- RLVR on tool-use agents is extremely sensitive to protocol equivalence.
- A verifier environment must reject the same malformed actions that eval rejects.
- Positive reward must match the final eval success condition exactly.
- Greedy canaries are useful because they reveal policy drift quickly.

The project does not currently support this claim:

```text
RLVR has improved the held-out Rust tool-use eval over SFT_HALF_A.
```

It has not.

---

## The practical lesson

For normal benchmark RLVR, the action is usually just "generate text" and the
grader consumes the final answer.

For tool-use RLVR, the action space is structured. The model is not just solving
Rust; it is speaking a protocol that controls a real environment.

That creates a harsher rule:

```text
If RL execution accepts actions that eval rejects, RL will eventually train the
model toward invalid actions.
```

The model is not being malicious. It is doing exactly what the reward channel
allows. If a malformed call still executes, the malformed call is part of the
training environment's action space.

That is the big debugging result so far.

---

## Next experiment

The next RLVR run should only be considered valid if all of the following hold:

1. The model sees the exact SFT/eval ChatML protocol during RL.
2. The RL parser rejects malformed CALL syntax before tool execution.
3. The only positive reward is strict held-out-style success.
4. Zero-advantage filtering remains on.
5. A greedy held-out canary is run early and often.
6. The final claim is measured only by held-out-69 `valid_trace`.

The target is still simple:

```text
RLVR checkpoint > SFT_HALF_A baseline on strict held-out-69 valid_trace
```

But now the experiment has a much better chance of being valid. The previous
runs were not just failed optimizations; they were harness audits. They showed
exactly where the environment was easier than the evaluator.

That is painful, but it is progress.
