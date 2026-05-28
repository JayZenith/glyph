#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import random
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from synthetic_data.validate_dataset import CANONICAL_FAMILIES, normalize_family

SYSTEM_PROMPT = "You are a Rust coding agent. Use tools when needed. After FINAL, stop immediately."
USER_RE = re.compile(r"<\|im_start\|>user\n(.*?)\n<\|im_end\|>", re.DOTALL)


def read_jsonl(path: Path) -> list[dict]:
    rows: list[dict] = []
    with path.open(encoding="utf-8") as handle:
        for line_no, raw in enumerate(handle, 1):
            raw = raw.strip()
            if not raw:
                continue
            try:
                rows.append(json.loads(raw))
            except json.JSONDecodeError as exc:
                raise SystemExit(f"{path}:{line_no}: invalid json: {exc}") from exc
    return rows


def user_from_trace(trace: str) -> str:
    match = USER_RE.search(trace)
    if not match:
        raise ValueError("trace missing user block")
    return match.group(1)


def parse_counts(value: str | None) -> dict[str, int] | None:
    if value is None:
        return None
    counts = json.loads(Path(value).read_text(encoding="utf-8")) if Path(value).exists() else json.loads(value)
    return {str(k): int(v) for k, v in counts.items()}


def select_rows(rows: list[dict], counts: dict[str, int] | None, seed: int) -> list[dict]:
    rng = random.Random(seed)
    by_family: dict[str, list[dict]] = defaultdict(list)
    for row in rows:
        family = normalize_family(row.get("family"))
        if family in CANONICAL_FAMILIES:
            by_family[family].append(row)
    for family_rows in by_family.values():
        rng.shuffle(family_rows)
    if counts is None:
        selected = [row for family in CANONICAL_FAMILIES for row in by_family.get(family, [])]
        rng.shuffle(selected)
        return selected
    selected: list[dict] = []
    missing: list[str] = []
    for family, count in counts.items():
        family_rows = by_family.get(family, [])
        if len(family_rows) < count:
            missing.append(f"{family}: have {len(family_rows)}, need {count}")
        selected.extend(family_rows[:count])
    if missing:
        raise SystemExit("not enough rows for eval counts:\n" + "\n".join(missing))
    rng.shuffle(selected)
    return selected


def build_rows(rows: list[dict], source_root: Path) -> list[dict]:
    eval_rows: list[dict] = []
    for row in rows:
        case_id = row["case_id"]
        blueprint_root = source_root / case_id
        if not blueprint_root.exists():
            raise SystemExit(f"missing blueprint for {case_id}: {blueprint_root}")
        eval_rows.append({
            "name": case_id,
            "kind": normalize_family(row["family"]),
            "system": SYSTEM_PROMPT,
            "blueprint_root": str(blueprint_root),
            "trace_prefix": f"runs/rlvr1/rust_cases/{case_id}",
            "user": user_from_trace(row["trace"]),
            "expected_tool_sequence": row["expected_tool_sequence"],
            "case_id": case_id,
            "difficulty": row.get("difficulty", "unknown"),
            "expected_output": row.get("expected_output"),
        })
    return eval_rows


def main() -> int:
    parser = argparse.ArgumentParser(description="Build eval prompt YAML from held-out materialized traces.")
    parser.add_argument("data", type=Path)
    parser.add_argument("--source-root", type=Path, default=Path("runs/rlvr1/rust_cases"))
    parser.add_argument("--output", type=Path, default=Path("sft/evals/generated_eval_prompts.yaml"))
    parser.add_argument("--section", default="post_eval")
    parser.add_argument("--counts", default=None, help="JSON object or JSON file of per-family eval counts")
    parser.add_argument("--seed", type=int, default=2026)
    args = parser.parse_args()

    rows = select_rows(read_jsonl(args.data), parse_counts(args.counts), args.seed)
    eval_rows = build_rows(rows, args.source_root)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(yaml.safe_dump({args.section: eval_rows}, sort_keys=False), encoding="utf-8")
    counts = Counter(row["kind"] for row in eval_rows)
    print(json.dumps({"wrote": str(args.output), "rows": len(eval_rows), "families": dict(sorted(counts.items()))}, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
