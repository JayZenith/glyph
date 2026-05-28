1. Reset

I previously had a DSL that was too overloaded for a 4B model:
- custom role wrappers
- plans
- thoughts
- satisfation bookkeeping
- tags
- refs
- proof markers
- extra Unicode operators

This new reset schema keeps only behavior that we should care about:
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
* We will also keep assistant-only masking and have the environment feed the system prompt, tools available, and tool results.

2. Simplifed Tool Scope

Keep the tool set narrow to avoid tool-choice confusion:
- `read_file`
- `apply_patch`
- `cargo_test`
- `cargo_run`

3. Synthetic Families

Preferred 1200 trace dataset mix and train on only these families:
320 patch_test_pass- `read_file -> apply_patch -> cargo_test -> FINAL`
180 patch_run_pass- `read_file -> apply_patch -> cargo_run -> FINAL`
340 patch_test_recover- `read_file -> apply_patch -> cargo_test_fail -> ... -> cargo_test_pass -> FINAL`
260 patch_run_recover- `read_file -> apply_patch -> cargo_run_fail -> ... -> cargo_run_pass -> FINAL`
60 test_only- `cargo_test -> FINAL`
30 run_only- `cargo_run -> FINAL`


* Preferred percentage mix:
  * pass families = `500 / 1200 = 41.7%`
  * recover families = `600 / 1200 = 50.0%`
  * single-tool families = `90 / 1200 = 7.5%`
* REMINDER: NEED TO MAKE SURE I GENERATE HARDER CASES WITHIN THE FAMILIES TO ENCOURAGE MULTI TURN USE.

*This complete set includes multi turn chat capabilities along with recovery behavior. We don't teach the model "failing answers are okay" but that failed tool results are feedback, and it should continue instead of giving up or hallucinating success.

* This is preferred over a more pass-heavy mix because it reduces dominance of easy one-shot repair and gives the model much more recovery practice before RL.

 * For scaling, i will keep one source file per family and merge into one train file at end. This allows me to scale easier as i can raise one family without touching others while auditing quality per family.
 - synthetic_data/families/patch_test_pass.jsonl
 - synthetic_data/families/patch_run_pass.jsonl
 - synthetic_data/families/patch_test_recover.jsonl
 - synthetic_data/families/patch_run_recover.jsonl
 - synthetic_data/families/test_only.jsonl
 - synthetic_data/families/run_only.jsonl

 The model should invent:

   - Rust bug
   - intended repair path
   - patch strings
   - user prompt
   - final message

   Local code should own:

   - file creation
   - tool execution
   - RESULT blocks
   - status pass/fail truth
   - final JSONL trace assembly
   - rejection of bad cases

   That gives you diversity from gpt-5.4, but correctness from execution. It also
   means bad generations are cheap to reject instead of silently poisoning SFT.


4. Evals
I wont include a random train/val/test split for this current run due to compute limits in generating a larger dataset, thus I will train on all curated traces but then will use a held-out post-eval that checks exact schemas, clean stopping, real tool success, and no extra tokens.

With scaling up, validation loss is useful for generalization/overfit monitorinng but here its weak as it does not mean "stops corectly" or "uses tool correctly"

The post-eval test will overweight the hard cases. I started with 8 held-out real tool-executed prompts to prove the loop works, and with scaling should push this much higher before wasting RL money.

Post-eval will follow this held-out curation mix:
40 `patch_test_pass` - `read_file -> apply_patch -> cargo_test -> FINAL`
30 `patch_run_pass` - `read_file -> apply_patch -> cargo_run -> FINAL`
60 `patch_test_recover` - `read_file -> apply_patch -> cargo_test_fail -> ... -> cargo_test_pass -> FINAL`
45 `patch_run_recover` - `read_file -> apply_patch -> cargo_run_fail -> ... -> cargo_run_pass -> FINAL`
10 `test_only` - `cargo_test -> FINAL`
5 `run_only` - `cargo_run -> FINAL`


# Prevoiusly i used mock tool calling, thats a cheap format gate. I will be implementing real tool calling in the post-sft eval to care about real patching, real recovery, and real verifier success.

# Real Example with real tool call eval suggestion
SFT eval can still be multi-turn with real tool execution as RL is about reward optimization, policy, updates, while this is just inference + execution + scoring.

  1. Start from the prompt.
  2. Let the model generate the next assistant block.
  3. If it emits a valid CALL ..., execute that tool for real.
  4. Insert the real RESULT cN: block.
  5. Let the model continue from there.
  6. Repeat until:
      - it emits FINAL: ...
      - or it hits max_tool_rounds
      - or it emits invalid structure

```bash
<|im_start|>system
You are a Rust coding agent. Use tools when needed. After FINAL, stop immediately.
<|im_end|>

<|im_start|>user
Fix the failing crate at runs/rlvr1/rust_cases/addlib_bug. Read src/lib.rs, patch the bug, run cargo_test, then answer once.
<|im_end|>

<|im_start|>assistant
CALL read_file(id=\"c1\", file_path=\"runs/rlvr1/rust_cases/addlib_bug/src/lib.rs\")
<|im_end|>

<|im_start|>tool
RESULT c1:\npub fn add(a: i32, b: i32) -> i32 { a - b }
<|im_end|>
```

``` bash
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
* My concern here is choosing a correct max_tool_rounds cap.


# Now really starting from a strong basis...

## The data pipeleine used to make sure we are not generating hallucinated tool traces
 I used GPT 5.4 to generate compact case specs from which we used local code execution to verify that the case specs were real for the following:
- Rust file creation
- tool execution
- real `RESULT` blocks

For dataset generation, I ran small pilot batches first, rejected bad generations, and only kept traces that survived real execution. From that process I assembled the current signal dataset:
- `synthetic_data/signal_259.jsonl`

On the data side I started with many brittle family splits, but simplified them into the ones that matter for scaling:
- `patch_test_pass`
- `patch_run_pass`
- `patch_test_recover`
- `patch_run_recover`
- `test_only`
- `run_only`


Current signal dataset counts:
- `patch_test_pass`: `105`
- `patch_run_pass`: `49`
- `patch_test_recover`: `95`
- `test_only`: `5`
- `run_only`: `5`

Validation became a required part of the workflow too. `synthetic_data/validate_dataset.py` now does both on the dataset:
- static protocol checks
- real execution of the traces

## SFT/RL/Evals alignment
I also aligned the runtime across synthetic data, SFT evals, and RL-facing code by using the same path style, same tool protocol, and shared runtime helpers. The eval loop is now real:
- model emits `CALL`
- tool executes for real
- real `RESULT` is injected
- model continues until `FINAL` or a hard cap

Then I built a held-out formal eval with real Rust cases and real tool execution. I also added contamination guards:
- exact prompt overlap hard fail via `--train-data`
- prompt similarity hard fail via `--max-prompt-similarity`

The best current SFT run on the 259 dataset is:
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

and the eval run on 8 tests was:
```bash
python -m sft.eval_formal \
    --sft-model runs/SIGNAL_259_SFT_E3_LR2E5/final \
    --train-data synthetic_data/signal_259.jsonl \
    --max-prompt-similarity 0.90 \
    --prompt-section post_eval \
    --output runs/SIGNAL_259_SFT_E3_LR2E5/eval_formal_post_eval_flatroots.json \
    --max-new-tokens 1200 \
    --max-tool-rounds 8 \
    --cases-root runs/rlvr1/rust_cases/eval
```


That run passed 6/8 tests from formal eval with actual tool execution. It is not finished, but it is enough proof that:
- the runtime works
- the dataset is teaching something real
- the eval is testing something real
- RLVR is more likely to be credible next step rather than a random gamble

Now it is time to scale the dataset and eval set up properly, because we have proof that the loop is working.


# Validation replays
We replay each savedtrace against Rust crate to prove trace is real
1. copy clean create/blueprint into a temp folder
2. run trace's tool calls in order
3. check actual results match saved RESULT blocks
4. Fail if patch/test/run differs.


OpenAI generated only `case specs`
* files: Cargo.toml, src/lib.rs or src/main.rs
* steps: read/patch/test/run
* expected output
* family metadata

Then `materialize_specs.py` used specs to
- create clean create blueprint under `runs/rlvr1/rust_cases/CASE_ID`
- copyp it to temp
- execute tools for real
- write final JSONL trace

IDs: the trace says paths like:
`runs/rlvr1/rust_cases/scale200_067_.../src/lib.rs`
Validation needs exact folder name to exist, or it cannot replay


# POTENTIAL ISSUE
- i was training only on `runs/rlvr1/rust_cases/...` which is too path-specific so going forwrad i need to randomize project roots.
- I didn't think of this as much since i was focused on working tool-use behavior.
- ACTUALLY, adding generalized file paths would add complexity over whats more important which is correct tool use


# Eval generation
Generated by submitting batch to GPT-5.4 to generate 100 held-out compact case specs and i will ask codex to generate the rest.


```bash
python3 synthetic_data/batch_specs.py retrieve \
  --metadata synthetic_data/eval_heldout_100/batch.json \
  --output synthetic_data/eval_heldout_100/results.jsonl
```
- this downlaods batch outputs from OpenAI and saves locally as `results.jsonl`
- then materialize them into real Rust crates and reject bad ones.

```bash
python3 synthetic_data/materialize_specs.py \
  synthetic_data/eval_heldout_100/results.jsonl \
  --source-root runs/rlvr1/rust_cases \
  --cases-root runs/materialize_eval_heldout_100_cases \
  --families-dir synthetic_data/eval_heldout_100_families \
  --rejects synthetic_data/eval_heldout_100/rejects.jsonl \
  --tool-timeout 30
```
- above creates blueprint crates under: `runs/rlvr1/rust_cases/CASE_ID` and disposable execution copies under: `runs/materialize_eval_heldout_100_cases/CASE_ID`

`CASE_ID` identifies specific Rust crate/problem. It appears in paths like:
- runs/rlvr1/rust_cases/CASE_ID/src/lib.rs
- runs/rlvr1/rust_cases/CASE_ID

so tools use it:
- CALL read_file(id="c1", file_path="runs/rlvr1/rust_cases/CASE_ID/src/lib.rs")
- CALL cargo_test(id="c3", project_path="runs/rlvr1/rust_cases/CASE_ID")

`cN` is different. It identifies the tool call/result pair:
` CALL read_file(id="c1", ...)
  RESULT c1:
`

so:
- CASE_ID = which Rust project
- c1,c2,c3 = wich tool call in the convo


# implemented rust-source similarity
removed 1 near-duplicate at .9328 filtered by >=.92
