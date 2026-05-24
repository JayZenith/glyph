#!/usr/bin/env python3
"""Build a user-prompt-unique RL-oriented SFT dataset.

Input:
- final_glyph_sft_dataset_rlvr_term_v3.jsonl

Output:
- final_glyph_sft_dataset_rlvr_term_v4_useruniq.jsonl

Policy:
- keep the first occurrence of each exact user prompt
- preserve row text otherwise
"""
from __future__ import annotations

import json
from collections import Counter
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "synthetic_data" / "final_glyph_sft_dataset_rlvr_term_v3.jsonl"
OUT = ROOT / "synthetic_data" / "final_glyph_sft_dataset_rlvr_term_v4_useruniq.jsonl"
REPORT = ROOT / "synthetic_data" / "final_glyph_sft_dataset_rlvr_term_v4_useruniq_report.json"


def extract_user(trace: str) -> str:
    marker = "<|im_start|>user\nuser「"
    end = "」🏷 usr1"
    start = trace.index(marker) + len(marker)
    stop = trace.index(end, start)
    return trace[start:stop]


def main() -> int:
    rows = [line for line in SRC.read_text(encoding="utf-8").splitlines() if line.strip()]
    seen_users: set[str] = set()
    kept: list[str] = []
    removed_users: Counter[str] = Counter()

    for row in rows:
        trace = json.loads(row)["trace"]
        user = extract_user(trace)
        if user in seen_users:
            removed_users[user] += 1
            continue
        seen_users.add(user)
        kept.append(row)

    OUT.write_text("\n".join(kept) + "\n", encoding="utf-8")
    report = {
        "source_rows": len(rows),
        "output_rows": len(kept),
        "removed_duplicate_user_rows": sum(removed_users.values()),
        "unique_users": len(seen_users),
        "source_file": str(SRC.relative_to(ROOT)),
        "output_file": str(OUT.relative_to(ROOT)),
    }
    REPORT.write_text(json.dumps(report, indent=2), encoding="utf-8")
    print(json.dumps(report, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
