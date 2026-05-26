"""Per-output scoring for the simplified CALL/RESULT/FINAL eval."""
from __future__ import annotations

import re
from collections import Counter, defaultdict


REPETITION_PATTERN = re.compile(r"(.{20,200}?)\1{4,}", re.DOTALL)
SEG_RE = re.compile(r"<\|im_start\|>(\w+)\n(.*?)<\|im_end\|>", re.DOTALL)
CALL_BLOCK_RE = re.compile(r"^\s*CALL\s+([A-Za-z_]\w*)\((.*?)\)\s*$", re.MULTILINE | re.DOTALL)
CALL_ID_RE = re.compile(r'\bid\s*=\s*"([^"]+)"')
RESULT_ID_RE = re.compile(r"^\s*RESULT\s+([A-Za-z0-9_\-]+):", re.MULTILINE)
ROLE_LEAK_RE = re.compile(r"(<\|im_start\|>|<\|im_end\|>|^\s*(system|user|assistant|tool)\s*$)", re.MULTILINE)


def _segments(text: str) -> list[tuple[str, str]]:
    return [(m.group(1), m.group(2)) for m in SEG_RE.finditer(text)]


def _assistant_bodies(full_trace: str) -> list[str]:
    return [body for role, body in _segments(full_trace) if role == "assistant"]


def _tool_bodies(full_trace: str) -> list[str]:
    return [body for role, body in _segments(full_trace) if role == "tool"]


def _extract_calls(assistant_bodies: list[str]) -> list[tuple[str, str]]:
    calls: list[tuple[str, str]] = []
    for body in assistant_bodies:
        for tool_name, arg_blob in CALL_BLOCK_RE.findall(body):
            match = CALL_ID_RE.search(arg_blob)
            if match:
                calls.append((tool_name, match.group(1)))
    return calls


def _extract_result_ids(tool_bodies: list[str]) -> list[str]:
    result_ids: list[str] = []
    for body in tool_bodies:
        result_ids.extend(RESULT_ID_RE.findall(body))
    return result_ids


def _failure_buckets(metrics: dict) -> list[str]:
    buckets: list[str] = []
    if not metrics["has_final"]:
        buckets.append("missing_final")
    if not metrics["clean_end"]:
        buckets.append("dirty_final")
    if not metrics["expected_tool_sequence_exact"]:
        buckets.append("wrong_tool_sequence")
    if not metrics["result_ids_match_call_ids"]:
        buckets.append("wrong_result_mapping")
    if not metrics["all_calls_have_ids"]:
        buckets.append("missing_call_ids")
    if metrics["role_marker_leakage"]:
        buckets.append("role_marker_leakage")
    if not metrics["no_repetition"]:
        buckets.append("repetition")
    if not metrics["not_truncated"]:
        buckets.append("truncated")
    if not metrics["final_after_last_tool"]:
        buckets.append("final_before_tool_completion")
    return buckets


def score_output(
    prompt_text: str,
    output_text: str,
    item: dict,
    new_token_count: int,
    max_new_tokens: int,
) -> dict:
    full_trace = prompt_text + output_text
    assistant_bodies = _assistant_bodies(full_trace)
    tool_bodies = _tool_bodies(full_trace)
    assistant_text = "\n".join(assistant_bodies)
    calls = _extract_calls(assistant_bodies)
    call_sequence = [tool for tool, _ in calls]
    call_ids = [call_id for _, call_id in calls]
    result_ids = _extract_result_ids(tool_bodies)
    expected_tool_sequence = item.get("expected_tool_sequence", [])
    final_blocks = [body for body in assistant_bodies if body.strip().startswith("FINAL:")]
    last_assistant = assistant_bodies[-1].strip() if assistant_bodies else ""

    metrics = {
        "kind": item.get("kind", "other"),
        "call_sequence": call_sequence,
        "call_ids": call_ids,
        "result_ids": result_ids,
        "expected_tool_sequence": expected_tool_sequence,
        "has_final": bool(final_blocks),
        "final_count": len(final_blocks),
        "clean_end": bool(assistant_bodies) and last_assistant.startswith("FINAL:"),
        "expected_tool_sequence_exact": call_sequence == expected_tool_sequence,
        "result_ids_match_call_ids": result_ids == call_ids[: len(result_ids)],
        "all_calls_have_ids": len(call_ids) == len(calls),
        "role_marker_leakage": bool(ROLE_LEAK_RE.search(assistant_text)),
        "no_repetition": REPETITION_PATTERN.search(assistant_text) is None,
        "not_truncated": new_token_count < max_new_tokens - 10,
        "new_token_count": new_token_count,
        "assistant_block_count": len(assistant_bodies),
        "tool_block_count": len(tool_bodies),
        "raw_chars": len(output_text),
    }

    if final_blocks:
        last_final_idx = max(i for i, body in enumerate(assistant_bodies) if body.strip().startswith("FINAL:"))
        last_call_idx = max((i for i, body in enumerate(assistant_bodies) if CALL_BLOCK_RE.search(body)), default=-1)
        metrics["final_after_last_tool"] = last_final_idx > last_call_idx
    else:
        metrics["final_after_last_tool"] = False

    metrics["valid_trace"] = (
        metrics["has_final"]
        and metrics["final_count"] == 1
        and metrics["clean_end"]
        and metrics["expected_tool_sequence_exact"]
        and metrics["result_ids_match_call_ids"]
        and metrics["all_calls_have_ids"]
        and metrics["final_after_last_tool"]
        and not metrics["role_marker_leakage"]
    )

    score = 0
    score += 4 if metrics["clean_end"] else 0
    score += 3 if metrics["expected_tool_sequence_exact"] else 0
    score += 2 if metrics["result_ids_match_call_ids"] else 0
    score += 1 if metrics["all_calls_have_ids"] else 0
    score += 1 if metrics["final_after_last_tool"] else 0
    score += 1 if metrics["no_repetition"] else 0
    score += 1 if metrics["not_truncated"] else 0
    metrics["score"] = score
    metrics["failure_buckets"] = _failure_buckets(metrics)
    return metrics


def summarize(name: str, rows: list[dict]) -> dict:
    total = len(rows)
    by_kind: dict[str, list[dict]] = defaultdict(list)
    failure_counts: Counter[str] = Counter()
    for row in rows:
        metrics = row["metrics"]
        by_kind[metrics["kind"]].append(row)
        failure_counts.update(metrics["failure_buckets"])

    kinds = {}
    for kind, kind_rows in sorted(by_kind.items()):
        n = len(kind_rows)
        kinds[kind] = {
            "num_prompts": n,
            "valid_trace_rate": sum(r["metrics"]["valid_trace"] for r in kind_rows) / n,
            "clean_end_rate": sum(r["metrics"]["clean_end"] for r in kind_rows) / n,
            "expected_tool_sequence_rate": sum(r["metrics"]["expected_tool_sequence_exact"] for r in kind_rows) / n,
            "result_id_match_rate": sum(r["metrics"]["result_ids_match_call_ids"] for r in kind_rows) / n,
            "final_after_last_tool_rate": sum(r["metrics"]["final_after_last_tool"] for r in kind_rows) / n,
        }

    return {
        "model": name,
        "num_prompts": total,
        "valid_traces": sum(1 for row in rows if row["metrics"]["valid_trace"]),
        "avg_score": sum(row["metrics"]["score"] for row in rows) / total,
        "clean_end_rate": sum(1 for row in rows if row["metrics"]["clean_end"]) / total,
        "expected_tool_sequence_rate": sum(1 for row in rows if row["metrics"]["expected_tool_sequence_exact"]) / total,
        "result_id_match_rate": sum(1 for row in rows if row["metrics"]["result_ids_match_call_ids"]) / total,
        "final_after_last_tool_rate": sum(1 for row in rows if row["metrics"]["final_after_last_tool"]) / total,
        "no_repetition_rate": sum(1 for row in rows if row["metrics"]["no_repetition"]) / total,
        "not_truncated_rate": sum(1 for row in rows if row["metrics"]["not_truncated"]) / total,
        "failure_buckets": dict(sorted(failure_counts.items())),
        "by_kind": kinds,
    }
