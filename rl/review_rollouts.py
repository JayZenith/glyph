#!/usr/bin/env python3
"""
Poll PRIME-RL rollout files and dump review samples every N steps.

This sidecar avoids depending on hidden PRIME-RL state internals. It reads the
actual rollout JSONL files after each completed step and writes compact review
files with stable exact-pair hashes for manual rejection.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import time
from pathlib import Path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output-dir", required=True)
    parser.add_argument("--review-dir", required=True)
    parser.add_argument("--every-steps", type=int, default=20)
    parser.add_argument("--count", type=int, default=8)
    parser.add_argument("--parent-pid", type=int, required=True)
    parser.add_argument("--poll-seconds", type=float, default=5.0)
    return parser.parse_args()


def canonical_json(value) -> str:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":"))


def review_key(prompt, completion) -> str:
    payload = canonical_json({"prompt": prompt, "completion": completion})
    return hashlib.sha256(payload.encode("utf-8")).hexdigest()


def parent_alive(pid: int) -> bool:
    try:
        os.kill(pid, 0)
    except OSError:
        return False
    return True


def dump_review_file(rollout_path: Path, review_path: Path, count: int, step: int) -> None:
    rows = []
    with rollout_path.open(encoding="utf-8") as f:
        for idx, line in enumerate(f):
            if idx >= count:
                break
            row = json.loads(line)
            row["review_step"] = step
            row["review_key"] = review_key(row.get("prompt"), row.get("completion"))
            rows.append(row)

    if not rows:
        return

    review_path.parent.mkdir(parents=True, exist_ok=True)
    with review_path.open("w", encoding="utf-8") as f:
        for row in rows:
            f.write(json.dumps(row, ensure_ascii=False) + "\n")


def main() -> int:
    args = parse_args()
    output_dir = Path(args.output_dir)
    review_dir = Path(args.review_dir)
    seen_steps: set[int] = set()

    while parent_alive(args.parent_pid):
        rollout_root = output_dir / "run_default" / "rollouts"
        if rollout_root.exists():
            for step_dir in sorted(rollout_root.glob("step_*")):
                try:
                    step = int(step_dir.name.split("_", 1)[1])
                except (IndexError, ValueError):
                    continue
                if step == 0 or step % args.every_steps != 0 or step in seen_steps:
                    continue
                rollout_path = step_dir / "train_rollouts.jsonl"
                if not rollout_path.exists():
                    continue
                review_path = review_dir / f"review_step_{step}.jsonl"
                dump_review_file(rollout_path, review_path, args.count, step)
                seen_steps.add(step)
        time.sleep(args.poll_seconds)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
