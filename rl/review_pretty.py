#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Pretty-print rollout review JSONL files.")
    parser.add_argument("path", help="Path to review_step_*.jsonl")
    parser.add_argument("--limit", type=int, default=8, help="Max rows to render.")
    parser.add_argument(
        "--completion-chars",
        type=int,
        default=5000,
        help="Max completion chars to print per rollout.",
    )
    parser.add_argument(
        "--show-prompt",
        action="store_true",
        help="Also print the prompt text.",
    )
    parser.add_argument(
        "--prompt-chars",
        type=int,
        default=1500,
        help="Max prompt chars to print when --show-prompt is set.",
    )
    return parser.parse_args()


def extract_text(value) -> str:
    if isinstance(value, str):
        return value
    if isinstance(value, list) and value:
        last = value[-1]
        if isinstance(last, dict):
            return str(last.get("content", ""))
    return str(value)


def trim(text: str, limit: int) -> str:
    if len(text) <= limit:
        return text
    return text[:limit] + "\n...<truncated>..."


def main() -> int:
    args = parse_args()
    path = Path(args.path)
    if not path.exists():
        raise FileNotFoundError(path)

    with path.open(encoding="utf-8") as f:
        for idx, line in enumerate(f, 1):
            if idx > args.limit:
                break
            row = json.loads(line)
            prompt = trim(extract_text(row.get("prompt", "")), args.prompt_chars)
            completion = trim(extract_text(row.get("completion", "")), args.completion_chars)

            print(f"\n==== Rollout {idx} ====")
            print(f"review_key: {row.get('review_key')}")
            print(f"reward: {row.get('reward')}")
            print(f"is_filtered: {row.get('is_filtered')}")
            print(f"is_completed: {row.get('is_completed')}")
            print(f"is_truncated: {row.get('is_truncated')}")
            print(f"stop_condition: {row.get('stop_condition')}")
            print(f"metrics: {json.dumps(row.get('metrics', {}), ensure_ascii=False)}")
            if args.show_prompt:
                print("\nprompt:\n")
                print(prompt)
            print("\ncompletion:\n")
            print(completion)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
