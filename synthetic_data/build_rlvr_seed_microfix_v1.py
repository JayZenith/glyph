#!/usr/bin/env python3
"""Build a tiny surgical SFT top-up for the remaining formal failures."""
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
from synthetic_data.build_gold_rust_tooluse import SYSTEM, rust_dev_tools, three_tool_trace  # noqa: E402


BASE_DATASET = ROOT / "synthetic_data" / "gold_glyph_3141_plus_rlvr_seed_final_v1.jsonl"
EVAL_PROMPTS = ROOT / "sft" / "evals" / "prompts_125.yaml"
TOPUP_OUT = ROOT / "synthetic_data" / "rlvr_seed_microfix_v1.jsonl"
COMBINED_OUT = ROOT / "synthetic_data" / "gold_glyph_3148_plus_rlvr_seed_microfix_v1.jsonl"
REPORT_OUT = ROOT / "synthetic_data" / "rlvr_seed_microfix_v1_report.json"


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


def cargo_diag_trace(
    name: str,
    user: str,
    project_path: str,
    stderr: str,
    response: str,
    note: str,
) -> str:
    todos = [
        f"Run cargo_check on {project_path} to inspect the compiler diagnostic.",
        "Give a concise diagnosis and first repair direction.",
    ]
    rationale = "The critical thing is to finish with the final diagnosis todo satisfied and a clean response."
    return g.join_trace(
        g.system_seg(
            "You are a Rust debugging assistant. Use the available tools sparingly and close the trace cleanly.",
            rust_dev_tools(),
        ),
        g.user_seg(user),
        g.assistant_seg(
            g.plan_block(todos, rationale),
            g.call_act("cargo_check", [("project_path", project_path)], f"chk_{name}", 1),
        ),
        g.result_seg(f"status: failure\\nexit_code: 101\\nstderr: {stderr}", f"chk_{name}"),
        g.assistant_seg(
            g.think_act([(note, f"note_{name}", [f"chk_{name}"])]),
            g.response_block(response, [f"chk_{name}", f"note_{name}"], 2),
        ),
    )


def plan_trace(
    name: str,
    user: str,
    availability: str,
    plan_result: str,
    response: str,
) -> str:
    return g.join_trace(
        g.system_seg(
            "You are a planning assistant who uses tools and always ends with one clean final response.",
            [
                g.tool(
                    "get_availability",
                    "Returns weekly availability by teammate.",
                    g.param("team_list", "string", "Comma-separated IDs", required=False),
                    g.param("start_date", "string", "Start date", required=False),
                    g.param("end_date", "string", "End date", required=False),
                ),
                g.tool(
                    "create_project_plan",
                    "Generates a short work breakdown.",
                    g.param("project_name", "string", "Project name", required=False),
                    g.param("objectives", "string", "Objectives", required=False),
                    g.param("timeline_weeks", "string", "Timeline in weeks", required=False),
                ),
            ],
        ),
        g.user_seg(user),
        g.assistant_seg(
            g.plan_block(
                [
                    "Check team availability for the rollout window.",
                    "Generate a high-level work breakdown.",
                    "Provide a prioritized schedule with launch checks.",
                ],
                "Get capacity first, then a work breakdown, then a clean final schedule.",
            ),
            g.call_act(
                "get_availability",
                [
                    ("team_list", "alex, beth, chris, diana"),
                    ("start_date", "2026-09-01"),
                    ("end_date", "2026-09-29"),
                ],
                f"avail_{name}",
                1,
            ),
        ),
        g.result_seg(availability, f"avail_{name}"),
        g.assistant_seg(
            g.think_act([("Availability is known, so generate the work breakdown next.", f"note_a_{name}", [f"avail_{name}"])]),
            g.call_act(
                "create_project_plan",
                [
                    ("project_name", "search filter redesign"),
                    ("objectives", "new UI, backend updates, A/B testing, docs"),
                    ("timeline_weeks", "4"),
                ],
                f"plan_{name}",
                2,
            ),
        ),
        g.result_seg(plan_result, f"plan_{name}"),
        g.assistant_seg(
            g.think_act([("Use the plan output to write the final schedule, then stop cleanly.", f"note_b_{name}", [f"avail_{name}", f"plan_{name}"])]),
            g.response_block(response, [f"avail_{name}", f"plan_{name}", f"note_b_{name}"], 3),
        ),
    )


def reverse_bin_trace(
    name: str,
    user: str,
    project_path: str,
    file_path: str,
    source: str,
    find: str,
    replace: str,
    stdout: str,
    response: str,
) -> str:
    todos = [
        f"Read {file_path} to find the exact buggy snippet.",
        "Apply one targeted patch.",
        f"Run cargo_run on {project_path} and report the stdout.",
    ]
    rationale = "Do not stop after apply_patch; the verifier call must happen before the final response."
    return three_tool_trace(
        user,
        todos,
        rationale,
        "read_file",
        [("file_path", file_path)],
        f"src_{name}",
        source.replace('"', '\\"'),
        "The source is short and the exact replacement text is visible.",
        f"note_src_{name}",
        "apply_patch",
        [("file_path", file_path), ("find", find), ("replace", replace)],
        f"patch_{name}",
        "status: success\\nexit_code: 0",
        "The patch succeeded, but the task is not complete until cargo_run verifies stdout.",
        f"note_patch_{name}",
        "cargo_run",
        [("project_path", project_path)],
        f"run_{name}",
        f"status: success\\nexit_code: 0\\nstdout: {stdout}",
        "The verifier output is now available, so the final response can close the trace.",
        f"note_run_{name}",
        response,
    )


def build_traces() -> list[str]:
    traces = [
        cargo_diag_trace(
            "recursive_type_box",
            "A Rust enum now fails to compile because a recursive variant makes the type size infinite. What is the first fix to check?",
            "/workspace/glyph/runs/rlvr1/rust_cases/recursive_type_case",
            "error[E0072]: recursive type `Node` has infinite size\\nhelp: insert some indirection such as `Box`, `Rc`, or `&` to break the cycle",
            "Inspect the recursive field and add indirection at that edge, usually with `Box<T>`. Recursive enums and structs need an indirection point so the compiler can give the type a finite size.",
            "The diagnostic already says the type needs indirection, so the response should close cleanly after that fix direction.",
        ),
        cargo_diag_trace(
            "recursive_type_traitobj",
            "Cargo check says a recursive Rust type has infinite size after a refactor. What should I inspect first?",
            "/workspace/glyph/runs/rlvr1/rust_cases/recursive_layout_case",
            "error[E0072]: recursive type `Expr` has infinite size\\nnote: recursive field without indirection",
            "Inspect the recursive field path and make sure one link uses indirection, commonly `Box<Expr>`. Without that indirection the compiler cannot lay out the recursive type at a finite size.",
            "The important thing is to satisfy the final diagnosis todo, not stop at the tool output.",
        ),
        plan_trace(
            "search_rollout",
            "We have four weeks to ship a search filter redesign. Check capacity, generate a work breakdown, then give me a prioritized rollout schedule with launch checks.",
            "alex: 60%\\nbeth: 80%\\nchris: 50%\\ndiana: 75%",
            "Week 1: UX/spec\\nWeek 2: frontend build\\nWeek 3: backend + integration\\nWeek 4: QA + docs + launch",
            "Prioritize the redesign as a four-week rollout: week 1 for UX/spec alignment, week 2 for frontend implementation, week 3 for backend and integration, and week 4 for QA, docs, and launch prep. Launch checks should cover cross-device UI validation, query latency, experiment readiness, and documentation completion.",
        ),
        plan_trace(
            "mobile_error_state",
            "Use the team availability and a generated plan to propose a four-week rollout for a mobile error-state redesign, then end with launch checks.",
            "alex: 70%\\nbeth: 70%\\nchris: 55%\\ndiana: 85%",
            "Week 1: design + instrumentation\\nWeek 2: UI implementation\\nWeek 3: backend wiring + metrics\\nWeek 4: QA, docs, rollout checks",
            "A reasonable four-week rollout is: week 1 for design and instrumentation, week 2 for UI implementation, week 3 for backend wiring and metrics, and week 4 for QA, docs, and rollout checks. Before launch, confirm the new states render correctly, telemetry is firing, backend fallbacks behave, and support docs are updated.",
        ),
        reverse_bin_trace(
            "reverse_collect",
            'The Cargo binary at "/workspace/glyph/runs/rlvr1/rust_cases/rev_collect_app" prints the string in the wrong order. Read the file, patch it, run cargo_run, and tell me the fixed stdout.',
            "/workspace/glyph/runs/rlvr1/rust_cases/rev_collect_app",
            "/workspace/glyph/runs/rlvr1/rust_cases/rev_collect_app/src/main.rs",
            'fn main() { let s = "rust"; let out: String = s.chars().collect(); println!("{out}"); }',
            "s.chars().collect()",
            "s.chars().rev().collect()",
            "tsur",
            'I added the missing iterator reversal, verified it with `cargo_run`, and the binary now prints `"tsur"`.',
        ),
        reverse_bin_trace(
            "reverse_join",
            'The binary at "/workspace/glyph/runs/rlvr1/rust_cases/rev_join_app" should print a reversed word. Inspect it, patch the bug, run cargo_run, and report stdout.',
            "/workspace/glyph/runs/rlvr1/rust_cases/rev_join_app",
            "/workspace/glyph/runs/rlvr1/rust_cases/rev_join_app/src/main.rs",
            'fn main() { let s = "hello"; let out: String = s.chars().collect(); println!("{out}"); }',
            "s.chars().collect()",
            "s.chars().rev().collect()",
            "olleh",
            'I patched the string construction, ran `cargo_run`, and confirmed the fixed stdout is `"olleh"`.',
        ),
        reverse_bin_trace(
            "reverse_chars",
            'Read "/workspace/glyph/runs/rlvr1/rust_cases/rev_chars_app/src/main.rs", apply one exact patch, then verify with cargo_run and tell me the corrected output.',
            "/workspace/glyph/runs/rlvr1/rust_cases/rev_chars_app",
            "/workspace/glyph/runs/rlvr1/rust_cases/rev_chars_app/src/main.rs",
            'fn main() { let s = "abcde"; let out: String = s.chars().collect(); println!("{out}"); }',
            "s.chars().collect()",
            "s.chars().rev().collect()",
            "edcba",
            'I inserted the missing reversal, verified the result with `cargo_run`, and the program now prints `"edcba"`.',
        ),
    ]
    return traces


def main() -> int:
    traces = build_traces()
    overlaps = sorted(set(extract_users(traces)) & load_eval_users())
    invalid = []
    for i, trace in enumerate(traces):
        res = validate_trace(trace)
        if not res.valid:
            invalid.append({"index": i, "errors": res.errors[:5]})
    if overlaps:
        print(json.dumps({"exact_prompt_overlaps": overlaps}, indent=2))
        return 1
    if invalid:
        print(json.dumps({"invalid": invalid}, indent=2))
        return 1
    with TOPUP_OUT.open("w", encoding="utf-8") as f:
        for trace in traces:
            f.write(json.dumps({"trace": trace}, ensure_ascii=False) + "\n")
    rows = [row for row in BASE_DATASET.read_text(encoding="utf-8").splitlines() if row.strip()]
    with COMBINED_OUT.open("w", encoding="utf-8") as f:
        for row in rows:
            f.write(row + "\n")
        for trace in traces:
            f.write(json.dumps({"trace": trace}, ensure_ascii=False) + "\n")
    report = {
        "base_rows": len(rows),
        "topup_rows": len(traces),
        "combined_rows": len(rows) + len(traces),
        "topup_file": str(TOPUP_OUT.relative_to(ROOT)),
        "combined_file": str(COMBINED_OUT.relative_to(ROOT)),
        "exact_prompt_overlap_count": 0,
    }
    REPORT_OUT.write_text(json.dumps(report, indent=2), encoding="utf-8")
    print(json.dumps(report, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
