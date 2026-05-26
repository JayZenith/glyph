# Agent Trace Schema

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

## Synthetic Trace Families

Use only these families in the reset SFT corpus.

### 1. `read_file -> apply_patch -> cargo_test -> FINAL`

```text
User: Fix the failing crate at /workspace/cases/addlib_bug. Read src/lib.rs, patch the bug, run cargo_test, then answer once.
Assistant: CALL read_file(id="c1", file_path="/workspace/cases/addlib_bug/src/lib.rs")
Tool: RESULT c1: pub fn add(a: i32, b: i32) -> i32 { a - b }
Assistant: CALL apply_patch(id="c2", file_path="/workspace/cases/addlib_bug/src/lib.rs", find="a - b", replace="a + b")
Tool: RESULT c2: status: success
Assistant: CALL cargo_test(id="c3", project_path="/workspace/cases/addlib_bug")
Tool: RESULT c3: status: success; stdout: test result: ok
Assistant: FINAL: Fixed the bug by changing subtraction to addition and verified it with cargo_test.
```

### 2. `read_file -> apply_patch -> cargo_run -> FINAL`

```text
User: Fix the crate at /workspace/cases/greeter_bug. Read src/main.rs, patch the greeting bug, run cargo_run, then answer once.
Assistant: CALL read_file(id="c1", file_path="/workspace/cases/greeter_bug/src/main.rs")
Tool: RESULT c1: fn main() { println!("Helo, world!"); }
Assistant: CALL apply_patch(id="c2", file_path="/workspace/cases/greeter_bug/src/main.rs", find="Helo", replace="Hello")
Tool: RESULT c2: status: success
Assistant: CALL cargo_run(id="c3", project_path="/workspace/cases/greeter_bug")
Tool: RESULT c3: status: success; stdout: Hello, world!
Assistant: FINAL: Fixed the greeting typo and verified the program output with cargo_run.
```

### 3. `read_file -> apply_patch -> cargo_test_fail -> read_file -> apply_patch -> cargo_test_pass -> FINAL`

```text
User: Fix the crate at /workspace/cases/range_bug. Read src/lib.rs, patch the bug, run cargo_test, and keep going until tests pass, then answer once.
Assistant: CALL read_file(id="c1", file_path="/workspace/cases/range_bug/src/lib.rs")
Tool: RESULT c1: pub fn sum_to(n: i32) -> i32 { (0..n).sum() }
Assistant: CALL apply_patch(id="c2", file_path="/workspace/cases/range_bug/src/lib.rs", find="0..n", replace="0..=n-1")
Tool: RESULT c2: status: success
Assistant: CALL cargo_test(id="c3", project_path="/workspace/cases/range_bug")
Tool: RESULT c3: status: failed; stderr: expected 15 got 10
Assistant: CALL read_file(id="c4", file_path="/workspace/cases/range_bug/src/lib.rs")
Tool: RESULT c4: pub fn sum_to(n: i32) -> i32 { (0..=n-1).sum() }
Assistant: CALL apply_patch(id="c5", file_path="/workspace/cases/range_bug/src/lib.rs", find="0..=n-1", replace="0..=n")
Tool: RESULT c5: status: success
Assistant: CALL cargo_test(id="c6", project_path="/workspace/cases/range_bug")
Tool: RESULT c6: status: success; stdout: test result: ok
Assistant: FINAL: Fixed the inclusive range bug and verified the passing tests with cargo_test.
```

### 4. `read_file -> apply_patch -> cargo_run_fail -> read_file -> apply_patch -> cargo_run_pass -> FINAL`

```text
User: Fix the crate at /workspace/cases/banner_bug. Read src/main.rs, patch the bug, run cargo_run, and keep going until the output is correct, then answer once.
Assistant: CALL read_file(id="c1", file_path="/workspace/cases/banner_bug/src/main.rs")
Tool: RESULT c1: fn main() { println!("welcom"); }
Assistant: CALL apply_patch(id="c2", file_path="/workspace/cases/banner_bug/src/main.rs", find="welcom", replace="Welcome")
Tool: RESULT c2: status: success
Assistant: CALL cargo_run(id="c3", project_path="/workspace/cases/banner_bug")
Tool: RESULT c3: status: failed; stdout: Welcome; stderr: expected exact output "Welcome!"
Assistant: CALL read_file(id="c4", file_path="/workspace/cases/banner_bug/src/main.rs")
Tool: RESULT c4: fn main() { println!("Welcome"); }
Assistant: CALL apply_patch(id="c5", file_path="/workspace/cases/banner_bug/src/main.rs", find="Welcome", replace="Welcome!")
Tool: RESULT c5: status: success
Assistant: CALL cargo_run(id="c6", project_path="/workspace/cases/banner_bug")
Tool: RESULT c6: status: success; stdout: Welcome!
Assistant: FINAL: Fixed the output string and verified the corrected runtime output with cargo_run.
```

### 5. `read_file -> apply_patch -> cargo_test_fail -> read_file -> apply_patch -> cargo_test_fail -> read_file -> apply_patch -> cargo_test_pass -> FINAL`

```text
User: Fix the crate at /workspace/cases/parser_bug. Read src/lib.rs, patch the bug, run cargo_test, and keep iterating until tests pass, then answer once.
Assistant: CALL read_file(id="c1", file_path="/workspace/cases/parser_bug/src/lib.rs")
Tool: RESULT c1: pub fn parse_num(s: &str) -> i32 { s.parse::<i32>().unwrap_or(1) }
Assistant: CALL apply_patch(id="c2", ... replace="unwrap_or(0)")
Tool: RESULT c2: status: success
Assistant: CALL cargo_test(id="c3", project_path="/workspace/cases/parser_bug")
Tool: RESULT c3: status: failed; stderr: parse_trimmed failed
Assistant: CALL read_file(id="c4", file_path="/workspace/cases/parser_bug/src/lib.rs")
Tool: RESULT c4: pub fn parse_num(s: &str) -> i32 { s.parse::<i32>().unwrap_or(0) }
Assistant: CALL apply_patch(id="c5", ... find="s.parse::<i32>()", replace="s.trim().parse::<i32>()")
Tool: RESULT c5: status: success
Assistant: CALL cargo_test(id="c6", project_path="/workspace/cases/parser_bug")
Tool: RESULT c6: status: failed; stderr: parse_signed failed
Assistant: CALL read_file(id="c7", file_path="/workspace/cases/parser_bug/src/lib.rs")
Tool: RESULT c7: pub fn parse_num(s: &str) -> i32 { s.trim().parse::<i32>().unwrap_or(0) }
Assistant: CALL apply_patch(id="c8", ... find="i32", replace="i64")
Tool: RESULT c8: status: success
Assistant: CALL cargo_test(id="c9", project_path="/workspace/cases/parser_bug")
Tool: RESULT c9: status: success; stdout: test result: ok
Assistant: FINAL: Fixed the parser across three iterations and verified the passing tests with cargo_test.
```

### 6. `read_file -> apply_patch -> cargo_run_fail -> read_file -> apply_patch -> cargo_run_fail -> read_file -> apply_patch -> cargo_run_pass -> FINAL`

```text
User: Fix the crate at /workspace/cases/counter_bug. Read src/main.rs, patch the bug, run cargo_run, and keep iterating until the output is correct, then answer once.
Assistant: CALL read_file(id="c1", file_path="/workspace/cases/counter_bug/src/main.rs")
Tool: RESULT c1: fn main() { for n in 1..4 { println!("{n}"); } }
Assistant: CALL apply_patch(id="c2", ... find="1..4", replace="1..=3")
Tool: RESULT c2: status: success
Assistant: CALL cargo_run(id="c3", project_path="/workspace/cases/counter_bug")
Tool: RESULT c3: status: failed; stdout: 1; stderr: expected 1 2 3 4
Assistant: CALL read_file(id="c4", file_path="/workspace/cases/counter_bug/src/main.rs")
Tool: RESULT c4: fn main() { for n in 1..=3 { println!("{n}"); } }
Assistant: CALL apply_patch(id="c5", ... find="1..=3", replace="1..=4")
Tool: RESULT c5: status: success
Assistant: CALL cargo_run(id="c6", project_path="/workspace/cases/counter_bug")
Tool: RESULT c6: status: failed; stdout: 1 2 3; stderr: missing final line 4
Assistant: CALL read_file(id="c7", file_path="/workspace/cases/counter_bug/src/main.rs")
Tool: RESULT c7: fn main() { for n in 1..=4 { println!("{n}"); } }
Assistant: CALL apply_patch(id="c8", ... find="println!(\"{n}\")", replace="println!(\"{n}\");")
Tool: RESULT c8: status: success
Assistant: CALL cargo_run(id="c9", project_path="/workspace/cases/counter_bug")
Tool: RESULT c9: status: success; stdout: 1 2 3 4
Assistant: FINAL: Fixed the counter output across three iterations and verified the final runtime output with cargo_run.
```

### 7. `cargo_test -> FINAL`

```text
User: Run cargo_test on /workspace/cases/passing_suite and report the result in one final answer.
Assistant: CALL cargo_test(id="c1", project_path="/workspace/cases/passing_suite")
Tool: RESULT c1: status: success; stdout: test result: ok. 6 passed; 0 failed
Assistant: FINAL: cargo_test passed for /workspace/cases/passing_suite with all tests succeeding.
```

### 8. `read_file -> FINAL`

```text
User: Read /workspace/cases/showcase_lib/src/lib.rs and summarize what it does in one final answer.
Assistant: CALL read_file(id="c1", file_path="/workspace/cases/showcase_lib/src/lib.rs")
Tool: RESULT c1: pub fn greet(name: &str) -> String { format!("Hello, {name}!") }
Assistant: FINAL: The file defines a greet function that returns a formatted hello string for the provided name.
```

## Post-Eval Families

The post-train eval set should include exactly one prompt for each of the eight families above. The source of truth lives in `sft/evals/eval_prompts.yaml`.
