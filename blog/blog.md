# SFT'ing Qwen3-4B-Base
1. Reset after failure on teaching Qwen3-4B-Base the structured agent trace

I previously had an agent trace language that was either too complicated for a 4B model or I had poor training runs:
* custom role wrappers, todos, thoughts, satisfaction bookkeeping, tags, refs, extra Unicode operators

To reduce complexity I stripped it down to the vital behavior:
- env/system role emits prompt
- User emits their prompt
- assistant emits `CALL ...`
- tool emits `RESULT ...`
- assistant emits exactly one `FINAL: ...`
- model stops immediately `<|im_end|>`

## Canonical Format
```bash
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

The model will be trained on assistant-only masking as the environment is responsible for responding to its tool calls via tool results.

2. Simplifying the Tool Scope

Keep the tool set narrow to avoid tool-choice confusion and keep important behavior such as reading files, editing files, running tests, and running programs:
* `read_file`, `apply_patch`, `cargo_test`, `cargo_run`

3. Building the Synthetic Dataset Families

Here's the preferred trace dataset mix and these families cover the main behavior as well as promoting multi-turn assistant behavior for error recovery:
patch_test_pass- `read_file -> apply_patch -> cargo_test -> FINAL`
patch_run_pass- `read_file -> apply_patch -> cargo_run -> FINAL`
patch_test_recover- `read_file -> apply_patch -> cargo_test_fail -> ... -> cargo_test_pass -> FINAL`
patch_run_recover- `read_file -> apply_patch -> cargo_run_fail -> ... -> cargo_run_pass -> FINAL`
test_only- `cargo_test -> FINAL`
run_only- `cargo_run -> FINAL`

* This is preferred over a more pass-heavy mix because it reduces dominance of easy one-shot repair and gives the model much more recovery practice before RL.

<!--* Preferred percentage mix:
  * pass families = `500 / 1200 = 41.7%`
  * recover families = `600 / 1200 = 50.0%`
  * single-tool families = `90 / 1200 = 7.5%`
* REMINDER: NEED TO MAKE SURE I GENERATE HARDER CASES WITHIN THE FAMILIES TO ENCOURAGE MULTI TURN USE.-->



 <!--* For scaling, I will keep one source file per family and merge into one train file at end. This allows me to scale easier as I can raise one family without touching others while auditing quality per family.
 - synthetic_data/families/patch_test_pass.jsonl
 - synthetic_data/families/patch_run_pass.jsonl
 - synthetic_data/families/patch_test_recover.jsonl
 - synthetic_data/families/patch_run_recover.jsonl
 - synthetic_data/families/test_only.jsonl
 - synthetic_data/families/run_only.jsonl-->


4. Running Evals
I won't include a random train/val/test split for this current run due to compute limits in generating a larger dataset but I will use a held-out post-eval that checks exact schemas, clean stopping, and real tool success. This post-eval should really gauge whether RLVR is viable.
* This post-eval also hard fails on exact overlap with the training dataset prompts and similarity between the training set crates and eval crates to avoid `data contamination` and measure `generalization`.
* This post-eval test will also overweight the hard cases from the dataset families. 

# Example with real tool call just focusing on actual behavior
SFT eval can still be multi-turn with real tool execution as RL is about reward optimization

```bash
<|im_start|>user
Fix the failing crate at runs/rlvr1/rust_cases/addlib_bug. Read src/lib.rs, patch the bug, run cargo_test, then answer once.
<|im_end|>

<|im_start|>assistant
CALL read_file(id=\"c1\", file_path=\"runs/rlvr1/rust_cases/addlib_bug/src/lib.rs\")
<|im_end|>

<|im_start|>tool
RESULT c1:\npub fn add(a: i32, b: i32) -> i32 { a - b }
<|im_end|>

<|im_start|>assistant
CALL apply_patch(id=\"c2\", file_path=\"runs/rlvr1/rust_cases/addlib_bug/src/lib.rs\", find=\"a - b\", replace=\"a + b\")\n<|im_end|>

<|im_start|>tool
RESULT c2:\nstatus: success\n<|im_end|>

<|im_start|>assistant
CALL cargo_test(id=\"c3\", project_path=\"runs/rlvr1/rust_cases/addlib_bug\")\n<|im_end|>

<|im_start|>tool
RESULT c3:\nstatus: success\nstdout: test result: ok\n<|im_end|>

<|im_start|>assistant
FINAL: Fixed the bug by changing subtraction to addition and verified it with cargo_test.
<|im_end|>
```

# Now really starting from a strong basis...
The pipeline to validate GPT 5.4 generated case specs through local code execution to verify crates are generated as well as real tool call results and valid expected behavior 
1. GPT generates a case spec
2. `materialize_specs.py` writes clean crate under source root.
3. It copies crate to sandbox
4. It executes intended tools on sandbox
5. If real execution matches expected status/stdout, writes training trace with real `RESULT` blocks.
6. Final train JSONL is merged.
7. `validate_dataset.py` later replays traces from clean blueprints into fresh sandbox copies.
8. Eval uses held-out blueprints and `eval_formal.py` to let model produce its own tool calls.


## SFT/RL/Evals alignment
- Training traces use `runs/rlvr1/rust_cases/CASE_ID`. 
- Eval prompts use the same style.
- Runtime rewrites that path to sandbox internally.
- This keeps model path format simple while making execution safe.
- It helped isolate the main question: can the model emit real tool use and repair behavior? YES. 

I initially passed 6/8 valid traces from formal eval which showed enough proof that:
- the runtime works
- the dataset is teaching something real
- the eval is testing something real

Then scaled up the dataset and eval set up properly to create an official `SFT_V1` model that will be carried into RLVR. 


## Rust-source similarity:
On top of making sure exact prompts don't overlap between eval and training set, I created a preflight command before eval that checked held-out eval crates are `not copies/near-copied` from train crates.

## The successful `SFT_V1` training run
The vital training specs:
- Epochs: 3
- Effective batch size: 8
- LR: 2e-5
- Max sequence length: 4096 (No Truncation)
- Masking: assistant-only
- No train split: trained on all curated traces
- Scheduler/defaults: cosine, warmup 0.03, weight decay 0.01, bf16/tf32 enabled
HF model: JayZenith/SFT_V1
HF dataset: JayZenith/SFT_V1_DATASET

## Key SFT_V1 findings showing RLVR readiness and what RLVR will look to fix 
- `52/69` valid traces with most failing on model not emitting a closing `FINAL`
  - Will look to solve this via RL
- Tool execution works: `68/69` terminal tool success 
- Format mostly works: call/result IDs `100%`, no repetition/truncation `100%`
- Real recovery works: example below fixed a bad patch after compiler error.
- Main remaining issue is policy/termination. `17` traces succeeded or kept trying but failed to emit clean `FINAL`.
- RLVR target is clear: reward verifier success + clean stop, penalize extra rounds/max-round loops


# REAL EVAL TRACE from `SFT_V1`
it reads, patches, gets compiler error, rereads, patches dereference `*enabled`, runs successfully, then emits `FINAL`

```bash
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


### cN used for tool call and result pairs
- CALL read_file(id="c1", file_path="runs/rlvr1/rust_cases/CASE_ID/src/lib.rs")
- CALL cargo_test(id="c3", project_path="runs/rlvr1/rust_cases/CASE_ID")

`cN` is different. It identifies the tool call/result pair:
` CALL read_file(id="c1", ...)
  RESULT c1:
`

so:
- CASE_ID = which Rust project
- c1,c2,c3 = which tool call in the convo



# So now moving onto RLVR ...
