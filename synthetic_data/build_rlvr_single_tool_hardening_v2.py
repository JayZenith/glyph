#!/usr/bin/env python3
"""Build a heavier single-tool sufficiency top-up for RL-oriented SFT.

This is a stronger follow-up to v1:
- much more explicit "do not read_file / do not patch / use exactly one tool"
- more surface-form variants per case
- intended to counter residual overuse of read_file and repair workflows
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
if str(ROOT / "synthetic_data") not in sys.path:
    sys.path.insert(0, str(ROOT / "synthetic_data"))

import build_gold50 as g  # noqa: E402
from core.validator import validate_trace  # noqa: E402
from synthetic_data.build_gold_rust_tooluse import SYSTEM, rust_dev_tools  # noqa: E402


BASE_DATASET = ROOT / "synthetic_data" / "final_glyph_sft_dataset_rlvr_term_v2.jsonl"
EVAL_PROMPTS = ROOT / "sft" / "evals" / "prompts_125.yaml"
TOPUP_OUT = ROOT / "synthetic_data" / "rlvr_seed_single_tool_hardening_v2.jsonl"
COMBINED_OUT = ROOT / "synthetic_data" / "final_glyph_sft_dataset_rlvr_term_v3.jsonl"
REPORT_OUT = ROOT / "synthetic_data" / "rlvr_seed_single_tool_hardening_v2_report.json"


def load_eval_users() -> set[str]:
    data = yaml.safe_load(EVAL_PROMPTS.read_text(encoding="utf-8"))
    rows = data["formal_eval"] if isinstance(data, dict) else data
    return {row["user"] for row in rows}


def extract_users(traces: list[str]) -> list[str]:
    users: list[str] = []
    marker = "<|im_start|>user\nuser「"
    end = "」🏷 usr1"
    for trace in traces:
        start = trace.index(marker) + len(marker)
        stop = trace.index(end, start)
        users.append(trace[start:stop])
    return users


READ_USER_TEMPLATES = [
    'Read "{path}", answer directly, and stop immediately after one final response.',
    'Open "{path}" and summarize it briefly. Do not call any extra tools after the file read.',
    'Inspect "{path}" with read_file, then respond once and terminate cleanly with no extra output.',
    'Use exactly one read_file call on "{path}", answer briefly, and stop right after the final response.',
    'This is a read-only task for "{path}". Do not patch, do not run cargo, just read once, answer, and stop.',
    'Call only read_file on "{path}", then give one concise response and emit nothing after it.',
    'Use one read_file call for "{path}" and no other tools. Answer once and terminate cleanly.',
    'Do not run builds or tests for "{path}". Read the file once, answer, and stop immediately.',
]

RUN_USER_TEMPLATES = [
    'Run the Cargo binary at "{path}" with cargo_run, report stdout briefly, and stop immediately after one final response.',
    'Use exactly one cargo_run call on "{path}", summarize the output, and do not read or patch any files.',
    'Execute "{path}" via cargo_run, answer once with the result, and terminate cleanly with no extra tool calls.',
    'Call cargo_run on "{path}", report the program output briefly, then stop right after the final response.',
    'This is a run-only task for "{path}". Do not read files, do not patch, just cargo_run once, answer, and stop.',
    'Use cargo_run exactly once on "{path}" and no other tools. Give one short response and terminate cleanly.',
    'Do not inspect source for "{path}". One cargo_run call is sufficient; answer from that result and stop.',
    'Skip read_file for "{path}". Run the binary once with cargo_run, report stdout, and end immediately.',
]

BUILD_USER_TEMPLATES = [
    'Run cargo_build on "{path}", report whether the build succeeds, and stop immediately after one final response.',
    'Use exactly one cargo_build call on "{path}", summarize the build outcome, and do not read or patch any files.',
    'Build "{path}" with cargo_build, answer once with the result, and terminate cleanly with no extra tool calls.',
    'Call cargo_build on "{path}", report the compile outcome briefly, then stop right after the final response.',
    'This is a build-only task for "{path}". Do not read files and do not patch anything; just cargo_build once, answer, and stop.',
    'Use cargo_build exactly once on "{path}" and no other tools. Give one short response and terminate cleanly.',
    'Do not inspect source for "{path}". One cargo_build call is sufficient; answer from that result and stop.',
    'Skip read_file for "{path}". Build once with cargo_build, report the result, and end immediately.',
]

CHECK_USER_TEMPLATES = [
    'Run cargo_check on "{path}", report the type-check result briefly, and stop immediately after one final response.',
    'Use exactly one cargo_check call on "{path}", summarize the diagnostic outcome, and do not read or patch any files.',
    'Check "{path}" with cargo_check, answer once with the result, and terminate cleanly with no extra tool calls.',
    'Call cargo_check on "{path}", report the compile diagnostics briefly, then stop right after the final response.',
    'This is a cargo_check-only task for "{path}". Do not read files, do not patch, and do not call rustdoc_lookup.',
    'Use cargo_check exactly once on "{path}" and no other tools. Give one short response and terminate cleanly.',
    'Do not inspect source for "{path}". One cargo_check call is sufficient; answer from that result and stop.',
    'Skip read_file and rustdoc for "{path}". Run cargo_check once, summarize the result, and end immediately.',
]

TEST_USER_TEMPLATES = [
    'Run cargo_test on "{path}", report the test outcome briefly, and stop immediately after one final response.',
    'Use exactly one cargo_test call on "{path}", summarize the test result, and do not read or patch any files.',
    'Test "{path}" with cargo_test, answer once with the result, and terminate cleanly with no extra tool calls.',
    'Call cargo_test on "{path}", report the suite outcome briefly, then stop right after the final response.',
    'This is a test-only task for "{path}". Do not read files and do not patch anything; just cargo_test once, answer, and stop.',
    'Use cargo_test exactly once on "{path}" and no other tools. Give one short response and terminate cleanly.',
    'Do not inspect source for "{path}". One cargo_test call is sufficient; answer from that result and stop.',
    'Skip read_file for "{path}". Run cargo_test once, summarize the outcome, and end immediately.',
]

RUSTC_USER_TEMPLATES = [
    'Compile "{source}" to "{output}" with rustc, report the compile outcome briefly, and stop immediately after one final response.',
    'Use exactly one rustc call on "{source}" with output "{output}", summarize the result, and do not read or patch any files.',
    'Build the standalone file "{source}" into "{output}" via rustc, answer once with the result, and terminate cleanly.',
    'Call rustc on "{source}" with output "{output}", report whether compilation succeeded, then stop right after the final response.',
    'This is a rustc-only task for "{source}". Do not read files, do not patch, just compile once, answer, and stop.',
    'Use rustc exactly once on "{source}" with output "{output}" and no other tools. Give one short response and terminate cleanly.',
    'Do not inspect source before compiling "{source}". One rustc call is sufficient; answer from that result and stop.',
    'Skip read_file for "{source}". Compile once with rustc to "{output}", report the result, and end immediately.',
]

RATIONALES = [
    "This task is satisfied by one tool call. Do not add read_file, apply_patch, rustdoc_lookup, or any extra verifier after the requested tool succeeds.",
    "Use exactly the requested tool once, then close with one clean final response and no trailing output.",
    "The minimal sufficient behavior is one correct tool call followed by a brief response and immediate termination.",
    "Do not expand this into a repair workflow. One tool result is enough, then the trace must stop cleanly.",
]

TOOL_THOUGHTS = [
    "This single tool result fully answers the task, so the response should close immediately after it.",
    "No extra inspection, patching, or second tool call is needed here; one tool call satisfies the task completely.",
    "The tool output now contains all the information required for the final response and clean termination.",
    "This is a single-tool task, so the trace should end after one brief response with no extra calls.",
]


def single_tool_hardened_trace(
    *,
    user: str,
    todo_1: str,
    todo_2: str,
    rationale: str,
    tool_name: str,
    args: list[tuple[str, str]],
    call_id: str,
    result_data: str,
    thought: str,
    note_tag: str,
    response: str,
) -> str:
    return g.join_trace(
        g.system_seg(SYSTEM, rust_dev_tools()),
        g.user_seg(user),
        g.assistant_seg(
            g.plan_block([todo_1, todo_2], rationale),
            g.call_act(tool_name, args, call_id, 1),
        ),
        g.result_seg(result_data, call_id),
        g.assistant_seg(
            g.think_act([(thought, note_tag, [call_id])]),
            g.response_block(response, [call_id, note_tag], 2),
        ),
    )


READ_CASES = [
    ("showcase_api", "/workspace/glyph/runs/rlvr1/rust_cases/showcase_lib/src/lib.rs",
     'pub fn greet(name: &str) -> String {\\n    format!("Hello, {}!", name)\\n}',
     [
         "The file defines a single public `greet` function that formats `Hello, <name>!`.",
         "It exposes one `greet` helper that returns a formatted hello string.",
     ]),
    ("inspect_main", "/workspace/glyph/runs/rlvr1/rust_cases/inspect_main_app/src/main.rs",
     'fn main() {\\n    let nums = [1, 2, 3, 4];\\n    let sum: i32 = nums.iter().sum();\\n    println!(\"{sum}\");\\n}',
     [
         "The program sums `1, 2, 3, 4` and prints the total.",
         "It computes the sum of the array values and prints that result.",
     ]),
    ("mathlib2", "/workspace/glyph/runs/rlvr1/rust_cases/mathlib2/src/lib.rs",
     "pub fn add(a: i32, b: i32) -> i32 { a + b }",
     [
         "The file defines a simple `add` function returning `a + b`.",
         "It exposes one public addition helper for two `i32` values.",
     ]),
    ("strutils", "/workspace/glyph/runs/rlvr1/rust_cases/strutils/src/lib.rs",
     "pub fn shout(s: &str) -> String { s.to_uppercase() }",
     [
         "The file defines `shout`, which uppercases its input string.",
         "It exposes one function that returns the uppercase version of the input.",
     ]),
]

RUN_CASES = [
    ("greeter2", "/workspace/glyph/runs/rlvr1/rust_cases/greeter2",
     "status: success\\nexit_code: 0\\nstdout: hello",
     [
         "cargo_run succeeded and the binary printed `hello`.",
         "The program ran successfully; stdout was `hello`.",
     ]),
    ("counter2", "/workspace/glyph/runs/rlvr1/rust_cases/counter2",
     "status: success\\nexit_code: 0\\nstdout: 1\\n2\\n3\\n4\\n5",
     [
         "cargo_run succeeded and printed the numbers `1` through `5` on separate lines.",
         "The program ran successfully; stdout was the sequence `1, 2, 3, 4, 5` line by line.",
     ]),
    ("rev_bin", "/workspace/glyph/runs/rlvr1/rust_cases/rev_bin",
     "status: success\\nexit_code: 0\\nstdout: olleh",
     [
         "cargo_run succeeded and the binary printed `olleh`.",
         "The program ran successfully; stdout was `olleh`.",
     ]),
    ("hello_bin", "/workspace/glyph/runs/rlvr1/rust_cases/hello_bin",
     "status: success\\nexit_code: 0\\nstdout: hello",
     [
         "cargo_run succeeded and the binary printed `hello`.",
         "The program ran successfully; stdout was `hello`.",
     ]),
]

BUILD_CASES = [
    ("greeter2", "/workspace/glyph/runs/rlvr1/rust_cases/greeter2",
     "status: success\\nexit_code: 0\\nstdout: Compiling greeter2 v0.1.0\\nFinished `dev` profile",
     [
         "cargo_build succeeded; the project compiled cleanly.",
         "The build finished successfully with no errors.",
     ]),
    ("counter2", "/workspace/glyph/runs/rlvr1/rust_cases/counter2",
     "status: success\\nexit_code: 0\\nstdout: Compiling counter2 v0.1.0\\nFinished `dev` profile",
     [
         "cargo_build succeeded; the binary compiled cleanly.",
         "The build completed successfully with no compile errors.",
     ]),
    ("release_pipeline", "/workspace/glyph/runs/rlvr1/rust_cases/release_pipeline",
     "status: success\\nexit_code: 0\\nstdout: Finished `dev` profile",
     [
         "cargo_build succeeded for the project.",
         "The build finished successfully.",
     ]),
    ("workspace_lint", "/workspace/glyph/runs/rlvr1/rust_cases/workspace_lint",
     "status: success\\nexit_code: 0\\nstdout: Finished `dev` profile",
     [
         "cargo_build succeeded for the workspace project.",
         "The workspace build completed successfully.",
     ]),
]

CHECK_CASES = [
    ("mathlib2", "/workspace/glyph/runs/rlvr1/rust_cases/mathlib2",
     "status: success\\nexit_code: 0\\nstdout: Checking mathlib2 v0.1.0\\nFinished",
     [
         "cargo_check passed; the crate type-checks cleanly.",
         "The project passed cargo_check with no diagnostics.",
     ]),
    ("optionlib_eval", "/workspace/glyph/runs/rlvr1/rust_cases/optionlib_eval",
     "status: success\\nexit_code: 0\\nstdout: Checking optionlib_eval v0.1.0\\nFinished",
     [
         "cargo_check passed; the project type-checks cleanly.",
         "The crate passed cargo_check without compile errors.",
     ]),
    ("traits_lib", "/workspace/glyph/runs/rlvr1/rust_cases/traits_lib",
     "status: failure\\nexit_code: 101\\nstderr: error[E0599]: no method named `run` found for struct `Runner` in the current scope",
     [
         "cargo_check failed with a missing-method diagnostic for `run` on `Runner`.",
         "The project did not pass cargo_check; the main diagnostic is that `Runner` has no `run` method in scope.",
     ]),
    ("recursive_type_case", "/workspace/glyph/runs/rlvr1/rust_cases/recursive_type_case",
     "status: failure\\nexit_code: 101\\nstderr: error[E0072]: recursive type `Node` has infinite size",
     [
         "cargo_check failed with a recursive-type infinite-size error for `Node`.",
         "The project did not pass cargo_check; the main diagnostic is an infinite-size recursive `Node` type.",
     ]),
]

TEST_CASES = [
    ("sortlib_eval", "/workspace/glyph/runs/rlvr1/rust_cases/sortlib_eval",
     "status: success\\nexit_code: 0\\nstdout: test result: ok",
     [
         "cargo_test passed; the test suite is green.",
         "The tests completed successfully with an `ok` result.",
     ]),
    ("dedup_lib", "/workspace/glyph/runs/rlvr1/rust_cases/dedup_lib",
     "status: success\\nexit_code: 0\\nstdout: test result: ok",
     [
         "cargo_test passed; the crate's tests succeed.",
         "The test suite completed successfully.",
     ]),
    ("gcdlib_bug_fixed", "/workspace/glyph/runs/rlvr1/rust_cases/gcdlib_bug",
     "status: success\\nexit_code: 0\\nstdout: test result: ok",
     [
         "cargo_test passed; the test run succeeded.",
         "The crate's tests completed successfully.",
     ]),
    ("maxlib_bug_fixed", "/workspace/glyph/runs/rlvr1/rust_cases/maxlib_bug",
     "status: success\\nexit_code: 0\\nstdout: test result: ok",
     [
         "cargo_test passed; the suite is green.",
         "The tests ran successfully with an `ok` result.",
     ]),
]

RUSTC_CASES = [
    ("hello2", "/workspace/glyph/runs/rlvr1/rust_cases/hello2.rs", "/workspace/glyph/runs/rlvr1/rust_cases/hello2_bin",
     "status: success\\nexit_code: 0\\nstdout:",
     [
         "rustc succeeded and wrote the requested output binary.",
         "The source compiled successfully to the requested output path.",
     ]),
    ("sum_one", "/workspace/glyph/runs/rlvr1/rust_cases/sum_one.rs", "/workspace/glyph/runs/rlvr1/rust_cases/sum_one_bin",
     "status: success\\nexit_code: 0\\nstdout:",
     [
         "rustc succeeded and produced the requested binary.",
         "The standalone source compiled successfully.",
     ]),
    ("hello_one", "/workspace/glyph/runs/rlvr1/rust_cases/hello_one.rs", "/workspace/glyph/runs/rlvr1/rust_cases/hello_one_bin",
     "status: success\\nexit_code: 0\\nstdout:",
     [
         "rustc succeeded and generated the requested executable.",
         "The file compiled successfully to the output binary.",
     ]),
    ("sum2", "/workspace/glyph/runs/rlvr1/rust_cases/sum2.rs", "/workspace/glyph/runs/rlvr1/rust_cases/sum2_bin",
     "status: success\\nexit_code: 0\\nstdout:",
     [
         "rustc succeeded and produced the output binary cleanly.",
         "The standalone file compiled successfully with no errors.",
     ]),
]


def _response(case_responses: list[str], variant_index: int) -> str:
    return case_responses[variant_index % len(case_responses)]


def _traces_for_two_arg_cases(cases, templates, tool_name, todo_template, note_prefix, arg_key):
    traces = []
    for case_index, case in enumerate(cases):
        name, path, result, responses = case
        for variant_index, template in enumerate(templates):
            traces.append(
                single_tool_hardened_trace(
                    user=template.format(path=path),
                    todo_1=todo_template.format(path=path),
                    todo_2="Report the outcome briefly, then stop immediately after one clean final response.",
                    rationale=RATIONALES[(case_index + variant_index) % len(RATIONALES)],
                    tool_name=tool_name,
                    args=[(arg_key, path)],
                    call_id=f"{tool_name}_{name}_{variant_index + 1}",
                    result_data=result.replace('"', '\\"') if tool_name == "read_file" else result,
                    thought=TOOL_THOUGHTS[(case_index + variant_index) % len(TOOL_THOUGHTS)],
                    note_tag=f"{note_prefix}_{name}_{variant_index + 1}",
                    response=_response(responses, variant_index),
                )
            )
    return traces


def build_traces() -> list[str]:
    traces = []
    traces.extend(_traces_for_two_arg_cases(
        READ_CASES, READ_USER_TEMPLATES, "read_file",
        "Use exactly one read_file call on {path}.",
        "note_read", "file_path",
    ))
    traces.extend(_traces_for_two_arg_cases(
        RUN_CASES, RUN_USER_TEMPLATES, "cargo_run",
        "Use exactly one cargo_run call on {path}.",
        "note_run", "project_path",
    ))
    traces.extend(_traces_for_two_arg_cases(
        BUILD_CASES, BUILD_USER_TEMPLATES, "cargo_build",
        "Use exactly one cargo_build call on {path}.",
        "note_build", "project_path",
    ))
    traces.extend(_traces_for_two_arg_cases(
        CHECK_CASES, CHECK_USER_TEMPLATES, "cargo_check",
        "Use exactly one cargo_check call on {path}.",
        "note_check", "project_path",
    ))
    traces.extend(_traces_for_two_arg_cases(
        TEST_CASES, TEST_USER_TEMPLATES, "cargo_test",
        "Use exactly one cargo_test call on {path}.",
        "note_test", "project_path",
    ))
    for case_index, case in enumerate(RUSTC_CASES):
        name, source, output, result, responses = case
        for variant_index, template in enumerate(RUSTC_USER_TEMPLATES):
            traces.append(
                single_tool_hardened_trace(
                    user=template.format(source=source, output=output),
                    todo_1=f"Use exactly one rustc call on {source} to produce {output}.",
                    todo_2="Report the compile outcome briefly, then stop immediately after one clean final response.",
                    rationale=RATIONALES[(case_index + variant_index) % len(RATIONALES)],
                    tool_name="rustc",
                    args=[("source_file", source), ("output", output)],
                    call_id=f"rustc_{name}_{variant_index + 1}",
                    result_data=result,
                    thought=TOOL_THOUGHTS[(case_index + variant_index) % len(TOOL_THOUGHTS)],
                    note_tag=f"note_rustc_{name}_{variant_index + 1}",
                    response=_response(responses, variant_index),
                )
            )
    return traces


def load_jsonl(path: Path) -> list[str]:
    return [line.strip() for line in path.read_text(encoding="utf-8").splitlines() if line.strip()]


def exact_dedupe(rows: list[str]) -> list[str]:
    seen = set()
    kept = []
    for row in rows:
        if row in seen:
            continue
        seen.add(row)
        kept.append(row)
    return kept


def count_rows_with_call(rows: list[str], tool_name: str) -> int:
    needle = f"tool ↦ {tool_name}"
    total = 0
    for row in rows:
        if needle in json.loads(row)["trace"]:
            total += 1
    return total


def main() -> int:
    traces = build_traces()
    overlaps = sorted(set(extract_users(traces)) & load_eval_users())
    invalid = []
    for i, trace in enumerate(traces):
        res = validate_trace(trace)
        if not res.valid:
            invalid.append({"index": i, "errors": res.errors[:5]})
    if overlaps:
        print(json.dumps({"exact_prompt_overlaps": overlaps[:10], "count": len(overlaps)}, indent=2))
        return 1
    if invalid:
        print(json.dumps({"invalid": invalid[:10], "count": len(invalid)}, indent=2))
        return 1

    topup_rows = [json.dumps({"trace": trace}, ensure_ascii=False) for trace in traces]
    base_rows = load_jsonl(BASE_DATASET)
    combined_rows = exact_dedupe(base_rows + topup_rows)

    TOPUP_OUT.write_text("\n".join(topup_rows) + "\n", encoding="utf-8")
    COMBINED_OUT.write_text("\n".join(combined_rows) + "\n", encoding="utf-8")

    report = {
        "base_rows": len(base_rows),
        "topup_rows": len(topup_rows),
        "combined_rows": len(combined_rows),
        "topup_tool_mix": {
            "read_file": count_rows_with_call(topup_rows, "read_file"),
            "cargo_run": count_rows_with_call(topup_rows, "cargo_run"),
            "cargo_build": count_rows_with_call(topup_rows, "cargo_build"),
            "cargo_check": count_rows_with_call(topup_rows, "cargo_check"),
            "cargo_test": count_rows_with_call(topup_rows, "cargo_test"),
            "rustc": count_rows_with_call(topup_rows, "rustc"),
        },
        "topup_file": str(TOPUP_OUT.relative_to(ROOT)),
        "combined_file": str(COMBINED_OUT.relative_to(ROOT)),
        "notes": [
            "This heavier top-up teaches exact single-tool sufficiency and clean immediate stopping.",
            "It is designed to reduce unnecessary read_file and repair-workflow expansion on single-tool tasks.",
        ],
    }
    REPORT_OUT.write_text(json.dumps(report, indent=2), encoding="utf-8")
    print(json.dumps(report, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
