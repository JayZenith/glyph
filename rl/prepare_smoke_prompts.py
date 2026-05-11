#!/usr/bin/env python3
"""
Create a small held-out prompt file for deterministic GRPO smoke tests.

This reproduces the original SFT 80/10/10 split from the full trace dataset,
selects rows from the held-out 10% test split, and writes prompt-only JSONL
records suitable for `rl/task_trace.load_environment`.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from datasets import Dataset


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", default="synthetic_data/sft_train_1098_official.jsonl")
    parser.add_argument("--output", default="runs/rl1/smoke_prompts_24.jsonl")
    parser.add_argument("--count", type=int, default=24)
    parser.add_argument("--split-seed", type=int, default=42)
    parser.add_argument("--sample-seed", type=int, default=42)
    return parser.parse_args()


def extract_prompt(trace: str) -> str:
    parts = trace.split("<|im_start|>assistant", 1)
    if len(parts) != 2:
        raise ValueError("trace does not contain an assistant segment")
    return parts[0] + "<|im_start|>assistant\n"


def main() -> int:
    args = parse_args()
    rows = []
    with Path(args.input).open(encoding="utf-8") as f:
        for idx, line in enumerate(f):
            item = json.loads(line)
            rows.append({"source_index": idx, "trace": item["trace"]})

    dataset = Dataset.from_list(rows)
    first = dataset.train_test_split(test_size=0.2, seed=args.split_seed)
    holdout = first["test"].train_test_split(test_size=0.5, seed=args.split_seed)
    test_split = holdout["test"]

    if args.count > len(test_split):
        raise ValueError(f"Requested {args.count} prompts but only {len(test_split)} held-out rows exist")

    selected = test_split.shuffle(seed=args.sample_seed).select(range(args.count))
    output_path = Path(args.output)
    output_path.parent.mkdir(parents=True, exist_ok=True)

    with output_path.open("w", encoding="utf-8") as f:
        for row in selected:
            record = {
                "prompt": extract_prompt(row["trace"]),
                "meta": {
                    "source_dataset": args.input,
                    "source_split": "sft_test_10pct",
                    "source_index": row["source_index"],
                    "split_seed": args.split_seed,
                    "sample_seed": args.sample_seed,
                },
            }
            f.write(json.dumps(record, ensure_ascii=False) + "\n")

    manifest = {
        "input": args.input,
        "output": str(output_path),
        "count": args.count,
        "source_split": "sft_test_10pct",
        "split_seed": args.split_seed,
        "sample_seed": args.sample_seed,
    }
    print(json.dumps(manifest, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
