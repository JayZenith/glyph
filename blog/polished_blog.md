# SFT'ing Qwen3-4B-Base Into a Rust Tool-Using Agent

This project started after a reset.

I previously had an agent trace language that was probably too complicated for a 4B model, or I had poor training runs. It had custom role wrappers, todos, thoughts, satisfaction bookkeeping, tags, refs, proof markers, and extra Unicode operators. That was too much surface area when the real question was much simpler:

Can a base 4B model learn to emit real tool calls, receive real tool results, patch Rust code, recover from compiler/test failures, and stop cleanly?

To reduce complexity, I stripped the format down to the vital behavior:

- system/user provide the task
- assistant emits `CALL ...`
- tool emits `RESULT ...`
- assistant emits exactly one `FINAL: ...`
- model stops immediately after `<|im_end|>`

## Canonical Format

```text
<|im_start|>system
You are a Rust coding agent. Use tools when needed. After FINAL, stop immediately.
<|im_end|>

<|im_start|>user
User task here.
<|im_end|>

<|im_start|>assistant
CALL tool_name(id="c1", arg1="...", arg2="...")
<|im_end|>

<|im_start|>tool
RESULT c1:
tool output here
<|im_end|>

<|im_start|>assistant
FINAL: brief final answer.
<|im_end|>
```

The model is trained with assistant-only masking because the environment is responsible for the system prompt, tool availability, and real tool results. The model should learn what the assistant controls: when to call tools, how to patch, how to react to failures, and when to stop.

## Keeping the Tool Scope Small

I kept the tool set narrow to avoid tool-choice confusion and focus on important coding-agent behavior:

- `read_file`
- `apply_patch`
- `cargo_test`
- `cargo_run`

This is enough to teach file inspection, editing, test execution, binary execution, and recovery from tool feedback.

## Synthetic Data Families

The dataset families were designed around behavior, not just task labels:

- `patch_test_pass`: `read_file -> apply_patch -> cargo_test -> FINAL`
- `patch_run_pass`: `read_file -> apply_patch -> cargo_run -> FINAL`
- `patch_test_recover`: `read_file -> apply_patch -> cargo_test_fail -> ... -> cargo_test_pass -> FINAL`
- `patch_run_recover`: `read_file -> apply_patch -> cargo_run_fail -> ... -> cargo_run_pass -> FINAL`
- `test_only`: `cargo_test -> FINAL`
- `run_only`: `cargo_run -> FINAL`

The important choice was not making the dataset too pass-heavy. Easy one-shot repair can dominate quickly, but the behavior I care about before RLVR is multi-turn recovery: failed tool results should become feedback, not hallucinated success.

## Generating Traces Without Hallucinated Tool Results

The key pipeline was using GPT-5.4 for case specs, then using local execution for truth.

GPT generated compact Rust case specs: the intended crate shape, source file, buggy code, intended patch path, tool sequence, expected behavior, and final response style. But GPT did not get to invent the final `RESULT` blocks.

Local code owned the important parts:

1. Build the Rust crate from the case spec.
2. Copy it into a sandbox.
3. Execute the intended tools for real.
4. Capture the real `stdout`, `stderr`, and status.
5. Reject cases that do not execute as expected.
6. Write the final JSONL trace using real tool results.
7. Replay traces later from clean blueprints into fresh sandbox copies.

That gives diversity from GPT-5.4, but correctness from execution. Bad generations are cheap to reject instead of silently poisoning SFT.

## SFT, RL, and Eval Alignment

I aligned training traces, formal evals, and RL-facing runtime around the same tool protocol and path style:

- Training traces use `runs/rlvr1/rust_cases/CASE_ID`.
- Eval prompts use the same style.
- Runtime rewrites that path to a sandbox copy internally.
- Clean blueprint crates stay immutable.
- Sandboxed case copies are disposable.

This keeps the model-facing path format simple while making execution safe. The point was to isolate the real question: can the model emit useful tool calls and repair behavior, not whether it can handle arbitrary filesystem complexity yet.

## Formal Eval

I did not rely on a random train/val/test split for the first serious run. Instead, I used a held-out post-eval with real Rust crates and real tool execution.

The eval checks:

- exact schema validity
- clean stopping after `FINAL`
- real tool success
- call/result ID consistency
- no repetition
- no truncation
- exact prompt overlap against training data
- Rust-source similarity between training crates and eval crates

This matters because validation loss alone does not tell you whether the model can stop correctly, call tools correctly, or recover from a real compiler error.

## SFT_V1 Training Run

The first serious model was trained as `SFT_V1`.

Vital training specs:

- Base model: `Qwen/Qwen3-4B-Base`
- Dataset: `synthetic_data/signal_1062.jsonl`
- Epochs: `3`
- Effective batch size: `8`
- LR: `2e-5`
- Max sequence length: `4096`
- Masking: assistant-only
- Train split: none, trained on all curated traces
- Scheduler/defaults: cosine, warmup `0.03`, weight decay `0.01`, bf16/tf32 enabled

Artifacts:

- HF model: `JayZenith/SFT_V1`
- HF dataset: `JayZenith/SFT_V1_DATASET`

## Key SFT_V1 Results

Held-out eval result:

- `52/69` valid traces
- `68/69` terminal tool success
- `100%` call/result ID consistency
- `100%` no repetition/truncation
- Main failure mode: the model often solved the task or kept working correctly, but failed to emit a clean final `FINAL`

That is the exact kind of signal I wanted before RLVR. Tool execution works. Recovery works. The main remaining issue is policy/termination: reward verifier success plus clean stop, penalize extra rounds and max-round loops.

## Real Eval Trace

This is the clearest example from `SFT_V1`: the model reads the file, patches it, hits a compiler error, reads again, fixes the dereference, reruns successfully, and emits `FINAL`.

```text
<|im_start|>system
You are a Rust coding agent. Use tools when needed. After FINAL, stop immediately.
<|im_end|>

<|im_start|>user
In the Rust crate at runs/rlvr1/rust_cases/eval100_024_patch_run_pass_005_active_tags_filter_map_join, fix the iterator pipeline in src/main.rs so running the binary prints the expected list of active tags.
<|im_end|>

<|im_start|>assistant
CALL read_file(id="c1", file_path="runs/rlvr1/rust_cases/eval100_024_patch_run_pass_005_active_tags_filter_map_join/src/main.rs")
<|im_end|>

<|im_start|>tool
RESULT c1:
status: success
stdout:
fn main() {
    let items = [
        ("core", true),
        ("draft", false),
        ("fast", true),
        ("wip", false),
        ("safe", true),
    ];

    let output = items
        .iter()
        .filter_map(|(name, enabled)| if !enabled { Some(*name) } else { None })
        .collect::<Vec<_>>()
        .join(",");

    println!("{}", output);
}
<|im_end|>

<|im_start|>assistant
CALL apply_patch(id="c2", file_path="runs/rlvr1/rust_cases/eval100_024_patch_run_pass_005_active_tags_filter_map_join/src/main.rs", find=".filter_map(|(name, enabled)| if !enabled { Some(*name) } else { None })", replace=".filter_map(|(name, enabled)| if enabled { Some(*name) } else { None })")
<|im_end|>

<|im_start|>tool
RESULT c2:
status: success
stdout:
patch applied
<|im_end|>

<|im_start|>assistant
CALL cargo_run(id="c3", project_path="runs/rlvr1/rust_cases/eval100_024_patch_run_pass_005_active_tags_filter_map_join")
<|im_end|>

<|im_start|>tool
RESULT c3:
status: failed
stderr:
error[E0308]: mismatched types
  --> src/main.rs:12:42
   |
12 |         .filter_map(|(name, enabled)| if enabled { Some(*name) } else { None })
   |                                          ^^^^^^^ expected `bool`, found `&bool`
   |
help: consider dereferencing the borrow
   |
12 |         .filter_map(|(name, enabled)| if *enabled { Some(*name) } else { None })
   |                                          +

For more information about this error, try `rustc --explain E0308`.
error: could not compile `active_tags_filter_map_join` (bin "active_tags_filter_map_join") due to previous error
<|im_end|>

<|im_start|>assistant
CALL read_file(id="c4", file_path="runs/rlvr1/rust_cases/eval100_024_patch_run_pass_005_active_tags_filter_map_join/src/main.rs")
<|im_end|>

<|im_start|>tool
RESULT c4:
status: success
stdout:
fn main() {
    let items = [
        ("core", true),
        ("draft", false),
        ("fast", true),
        ("wip", false),
        ("safe", true),
    ];

    let output = items
        .iter()
        .filter_map(|(name, enabled)| if enabled { Some(*name) } else { None })
        .collect::<Vec<_>>()
        .join(",");

    println!("{}", output);
}
<|im_end|>

<|im_start|>assistant
CALL apply_patch(id="c5", file_path="runs/rlvr1/rust_cases/eval100_024_patch_run_pass_005_active_tags_filter_map_join/src/main.rs", find=".filter_map(|(name, enabled)| if enabled { Some(*name) } else { None })", replace=".filter_map(|(name, enabled)| if *enabled { Some(*name) } else { None })")
<|im_end|>

<|im_start|>tool
RESULT c5:
status: success
stdout:
patch applied
<|im_end|>

<|im_start|>assistant
CALL cargo_run(id="c6", project_path="runs/rlvr1/rust_cases/eval100_024_patch_run_pass_005_active_tags_filter_map_join")
<|im_end|>

<|im_start|>tool
RESULT c6:
status: success
stdout:
core,fast,safe
<|im_end|>

<|im_start|>assistant
FINAL: Fixed the iterator filter_map condition so it keeps active tags and dereferences the boolean flag, producing the expected stdout.
<|im_end|>
```

## CASE_ID vs cN

`CASE_ID` identifies the Rust project:

```text
runs/rlvr1/rust_cases/CASE_ID
```

`cN` identifies the tool call/result pair inside the conversation:

```text
CALL read_file(id="c1", file_path="runs/rlvr1/rust_cases/CASE_ID/src/lib.rs")
RESULT c1:
```

So:

- `CASE_ID` = which Rust crate/project
- `c1`, `c2`, `c3` = which tool call in the conversation

## Why This Makes RLVR the Next Step

This is not just format SFT. The model learned a usable interaction loop:

- inspect code
- patch code
- run Rust tools
- read compiler/test feedback
- recover from a bad patch
- produce a final answer

The remaining failure mode is exactly what RLVR is good at optimizing: reward actual verifier success, reward clean finalization, and penalize loops or malformed endings.

The next step is scaling both the dataset and the held-out eval set, then running RLVR on top of `SFT_V1`.
