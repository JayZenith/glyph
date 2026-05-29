#!/usr/bin/env python3
"""Per-step rollout quality summary for a CALL/RESULT/FINAL RL run."""
from __future__ import annotations

import argparse
import glob
import json
import os
import statistics
from pathlib import Path


def rollout_paths(run_dir: Path) -> list[str]:
    rollouts_dir = run_dir / "run_default" / "rollouts"
    if not rollouts_dir.exists():
        rollouts_dir = run_dir / "rollouts"
    pattern = str(rollouts_dir / "step_*" / "train_rollouts.jsonl")
    return sorted(
        glob.glob(pattern),
        key=lambda path: int(os.path.basename(os.path.dirname(path)).split("_")[1]),
    )


def summarize(path: str) -> tuple:
    rewards: list[float] = []
    tools = no_call = final = zeros = pos = 0
    lengths: list[int] = []

    with open(path, encoding="utf-8") as handle:
        for raw in handle:
            row = json.loads(raw)
            reward = float(row.get("reward", 0.0))
            rewards.append(reward)
            pos += reward > 0

            assistant = "\n".join(
                message.get("content", "")
                for message in row.get("completion", [])
                if isinstance(message, dict) and message.get("role") == "assistant"
            )
            no_call += "CALL " not in assistant
            final += "FINAL:" in assistant
            tools += sum(
                1
                for message in row.get("completion", [])
                if isinstance(message, dict) and message.get("role") == "tool"
            )
            zeros += bool(row.get("filters", {}).get("zero_advantage"))
            lengths.append(len(assistant))

    step = int(os.path.basename(os.path.dirname(path)).split("_")[1])
    return (
        step,
        statistics.mean(rewards),
        min(rewards),
        max(rewards),
        pos,
        no_call,
        final,
        tools,
        zeros,
        round(statistics.mean(lengths)),
    )


def main() -> None:
    parser = argparse.ArgumentParser(description="Summarize task-trace rollout quality.")
    parser.add_argument(
        "run_dir",
        nargs="?",
        default="outputs/rlvr1",
        type=Path,
        help="Run directory containing run_default/rollouts.",
    )
    parser.add_argument("--tail", type=int, default=20, help="Number of recent steps to print.")
    args = parser.parse_args()

    rows = [summarize(path) for path in rollout_paths(args.run_dir)]
    print("step avg min max pos no_call final tools zero len")
    for row in rows[-args.tail :]:
        print(row[0], round(row[1], 4), round(row[2], 2), round(row[3], 2), *row[4:])
    if rows:
        latest = rows[-10:]
        print(
            "summary latest10",
            "avg_reward",
            round(statistics.mean(row[1] for row in latest), 4),
            "avg_pos",
            round(statistics.mean(row[4] for row in latest), 2),
            "avg_no_call",
            round(statistics.mean(row[5] for row in latest), 2),
            "avg_final",
            round(statistics.mean(row[6] for row in latest), 2),
            "avg_tools",
            round(statistics.mean(row[7] for row in latest), 2),
            "avg_len",
            round(statistics.mean(row[9] for row in latest)),
        )


if __name__ == "__main__":
    main()
