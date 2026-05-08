"""Per-output scoring + per-model summary for the formal generation eval."""
import re

from core.validator import validate_trace


REPETITION_PATTERN = re.compile(r"(.{20,200}?)\1{4,}", re.DOTALL)
TAIL_OK_PATTERN = re.compile(r"[\s※⊨𝑝🏷•\[\]\w\d\.\-\"']*")


def score_output(
    prompt_text: str,
    output_text: str,
    tools: list[dict],
    new_token_count: int,
    max_new_tokens: int,
) -> dict:
    full_trace = prompt_text + output_text
    validation = validate_trace(full_trace)
    metrics = {
        "valid_trace": validation.valid,
        "warning_count": len(validation.warnings),
        "error_count": len(validation.errors),
        "has_plan": "plan {" in output_text,
        "has_act": "act {" in output_text,
        "has_response": "response「" in output_text or 'response"' in output_text,
        "has_result": "result {" in output_text,
        "has_tool_call": "call ↦ {" in output_text,
        "has_think_block": "think ↦" in output_text,
        "raw_chars": len(output_text),
    }
    if tools:
        tool_names = [tool["name"] for tool in tools]
        metrics["mentions_any_tool_name"] = any(name in output_text for name in tool_names)
    else:
        metrics["mentions_any_tool_name"] = False

    metrics["no_repetition"] = REPETITION_PATTERN.search(output_text) is None
    last_resp = output_text.rfind("response「")
    last_close = output_text.rfind("」")
    tail = output_text[last_close + 1:].strip() if last_close >= 0 else ""
    tail_ok = bool(TAIL_OK_PATTERN.fullmatch(tail))
    metrics["ends_with_response"] = last_resp >= 0 and last_close > last_resp and tail_ok
    metrics["not_truncated"] = new_token_count < max_new_tokens - 10
    metrics["new_token_count"] = new_token_count

    score = 0
    score += 3 if metrics["valid_trace"] and metrics["no_repetition"] else 0
    score += 1 if metrics["has_plan"] else 0
    score += 1 if metrics["has_response"] and metrics["ends_with_response"] else 0
    score += 1 if (metrics["has_tool_call"] if tools else metrics["has_think_block"]) else 0
    score += 1 if metrics["not_truncated"] else 0
    metrics["score"] = score
    metrics["validation_errors"] = validation.errors
    metrics["validation_warnings"] = validation.warnings
    return metrics


def summarize(name: str, rows: list[dict]) -> dict:
    total = len(rows)
    return {
        "model": name,
        "num_prompts": total,
        "valid_traces": sum(1 for row in rows if row["metrics"]["valid_trace"]),
        "avg_score": sum(row["metrics"]["score"] for row in rows) / total,
        "has_plan_rate": sum(1 for row in rows if row["metrics"]["has_plan"]) / total,
        "has_response_rate": sum(1 for row in rows if row["metrics"]["has_response"]) / total,
        "has_tool_call_rate": sum(1 for row in rows if row["metrics"]["has_tool_call"]) / total,
        "no_repetition_rate": sum(1 for row in rows if row["metrics"]["no_repetition"]) / total,
        "ends_with_response_rate": sum(1 for row in rows if row["metrics"]["ends_with_response"]) / total,
        "not_truncated_rate": sum(1 for row in rows if row["metrics"]["not_truncated"]) / total,
    }
