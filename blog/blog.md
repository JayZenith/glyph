1. Reset

I previously had a DSL that was too overloaded for a 4B model:
- custom role wrappers
- plans
- thoughts
- tags
- refs
- proof markers
- extra Unicode operators

The reset schema keeps only behavior that matters:
- assistant emits `CALL ...`
- tool emits `RESULT ...`
- assistant emits exactly one `FINAL: ...`
- model stops immediately

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

2. Tool Scope

Keep the tool set narrow:
- `read_file`
- `apply_patch`
- `cargo_test`
- `cargo_run`

3. Synthetic Families

Train on only these families:
- `read_file -> apply_patch -> cargo_test -> FINAL`
- `read_file -> apply_patch -> cargo_run -> FINAL`
- `read_file -> apply_patch -> cargo_test_fail -> read_file -> apply_patch -> cargo_test_pass -> FINAL`
- `read_file -> apply_patch -> cargo_run_fail -> read_file -> apply_patch -> cargo_run_pass -> FINAL`
- `read_file -> apply_patch -> cargo_test_fail -> read_file -> apply_patch -> cargo_test_fail -> read_file -> apply_patch -> cargo_test_pass -> FINAL`
- `read_file -> apply_patch -> cargo_run_fail -> read_file -> apply_patch -> cargo_run_fail -> read_file -> apply_patch -> cargo_run_pass -> FINAL`
- `cargo_test -> FINAL`
- `read_file -> FINAL`

4. First Eval Set

The post-train eval should mirror the same eight families. It now lives in:
- `sft/evals/eval_prompts.yaml`
