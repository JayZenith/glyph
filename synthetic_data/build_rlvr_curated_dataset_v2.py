#!/usr/bin/env python3
"""Build a stricter curated RL-only SFT dataset.

Compared with curated_v1 this version:
- uses only the termination-hardening seed set and the stronger single-tool v2 top-up
- enforces exact allowed tool sequences
- drops traces that fail structural integrity checks even if they came from approved sources
"""
from __future__ import annotations

import json
import re
import sys
from collections import Counter
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.validator import validate_trace

SOURCES = [
    ROOT / "synthetic_data" / "rlvr_seed_termination_hardening_v1.jsonl",
    ROOT / "synthetic_data" / "rlvr_seed_single_tool_hardening_v2.jsonl",
]
PROMPTS_FILE = ROOT / "sft" / "evals" / "prompts_125.yaml"
OUT = ROOT / "synthetic_data" / "final_glyph_sft_dataset_rlvr_curated_v2.jsonl"
REPORT = ROOT / "synthetic_data" / "final_glyph_sft_dataset_rlvr_curated_v2_report.json"

USER_RE = re.compile(r"<\|im_start\|>user\nuser「(.*?)」🏷 usr1", re.DOTALL)
SEG_RE = re.compile(r"<\|im_start\|>(\w+)\n(.*?)<\|im_end\|>", re.DOTALL)
CALL_TOOL_RE = re.compile(r"call\s*↦\s*\{[^}]*?\btool\s*↦\s*([^\s•}]+)", re.DOTALL)
TAIL_OK_RE = re.compile(r"[\s※⊨𝑝🏷•\[\]\w\d\.\-\"']*")

ALLOWED_SEQUENCES = {
    ("read_file", "apply_patch", "cargo_test"),
    ("read_file", "apply_patch", "cargo_run"),
    ("read_file",),
    ("cargo_build",),
    ("cargo_check",),
    ("cargo_run",),
    ("cargo_test",),
    ("rustc",),
}


def load_rows() -> list[dict]:
    rows: list[dict] = []
    for path in SOURCES:
        with path.open(encoding="utf-8") as fh:
            for line in fh:
                line = line.strip()
                if line:
                    rows.append(json.loads(line))
    return rows


def extract_user(trace: str) -> str:
    match = USER_RE.search(trace)
    if not match:
        raise ValueError("missing user segment")
    return match.group(1)


def load_formal_eval_rl_users() -> set[str]:
    obj = yaml.safe_load(PROMPTS_FILE.read_text(encoding="utf-8"))
    formal = {row["name"]: row["user"] for row in obj["formal_eval"]}
    spec = obj["formal_eval_rl"]
    return {formal[name] for name in spec["names"]}


def assistant_text(trace: str) -> str:
    return "\n".join(body for role, body in SEG_RE.findall(trace) if role == "assistant")


def clean_end(text: str) -> bool:
    last_resp = text.rfind("response「")
    last_close = text.rfind("」")
    tail = text[last_close + 1 :].strip() if last_close >= 0 else ""
    return last_resp >= 0 and last_close > last_resp and bool(TAIL_OK_RE.fullmatch(tail))


def integrity_failure(trace: str) -> str | None:
    validation = validate_trace(trace)
    if not validation.valid:
        return "validator_invalid"

    assistant = assistant_text(trace)
    seq = tuple(tool.strip('"') for tool in CALL_TOOL_RE.findall(assistant))
    if seq not in ALLOWED_SEQUENCES:
        return "tool_sequence_not_allowed"

    if assistant.count("response「") != 1:
        return "response_count_not_one"

    if not clean_end(assistant):
        return "not_clean_end"

    if "<|im_start|>" in assistant or "<|im_end|>" in assistant:
        return "nested_role_marker_in_assistant"

    if "rustdoc_lookup" in assistant:
        return "rustdoc_lookup_present"

    if re.search(r"(^|\n)\s*(system|user|assistant|tool)\s*$", assistant):
        return "role_label_leakage"

    if "pomięd" in assistant:
        return "known_garbage_token"

    return None


def main() -> None:
    rows = load_rows()
    exact_seen: set[str] = set()
    user_seen: set[str] = set()
    kept: list[dict] = []
    dropped_reasons: Counter[str] = Counter()

    for row in rows:
        key = json.dumps(row, ensure_ascii=False, sort_keys=True)
        if key in exact_seen:
            dropped_reasons["exact_duplicate_row"] += 1
            continue
        exact_seen.add(key)

        user = extract_user(row["trace"])
        if user in user_seen:
            dropped_reasons["duplicate_user_prompt"] += 1
            continue

        failure = integrity_failure(row["trace"])
        if failure is not None:
            dropped_reasons[failure] += 1
            continue

        user_seen.add(user)
        kept.append(row)

    seq_counts = Counter()
    for row in kept:
        seq = tuple(tool.strip('"') for tool in CALL_TOOL_RE.findall(assistant_text(row["trace"])))
        seq_counts[" -> ".join(seq)] += 1

    eval_overlap = sorted(load_formal_eval_rl_users().intersection(user_seen))

    with OUT.open("w", encoding="utf-8") as fh:
        for row in kept:
            fh.write(json.dumps(row, ensure_ascii=False) + "\n")

    report = {
        "source_files": [str(path.relative_to(ROOT)) for path in SOURCES],
        "source_rows": len(rows),
        "output_rows": len(kept),
        "dropped_reasons": dict(sorted(dropped_reasons.items())),
        "unique_users": len(user_seen),
        "formal_eval_rl_exact_user_overlap": len(eval_overlap),
        "tool_sequences": dict(sorted(seq_counts.items())),
        "output_file": str(OUT.relative_to(ROOT)),
    }
    REPORT.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
