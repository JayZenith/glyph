#!/usr/bin/env python3
from __future__ import annotations

import argparse
import asyncio
import json
import math
import statistics
import sys
import types
import unittest
from collections import Counter, defaultdict
from pathlib import Path

from agent_runtime.protocol import ended_cleanly_after_final, parse_calls


ROLLOUT_PATHS = [
    Path("results/RLVR1/rollouts4/step_0/train_rollouts.jsonl"),
    Path("results/RLVR1/rollouts4/step_95/train_rollouts.jsonl"),
]
VERIFIERS = {"cargo_test", "cargo_run"}


def _install_verifiers_stub() -> None:
    if "verifiers" in sys.modules:
        return
    vf = types.ModuleType("verifiers")
    vf.MultiTurnEnv = type("MultiTurnEnv", (), {"__init__": lambda self, *a, **k: None})
    vf.Rubric = type(
        "Rubric",
        (),
        {"__init__": lambda self, *a, **k: None, "add_reward_func": lambda self, *a, **k: None},
    )
    sys.modules["verifiers"] = vf


_install_verifiers_stub()
from rl.task_trace import _find_result_for, _rust_tool_reward, _verifier_outcomes  # noqa: E402


def result_block(call_id: str, success: bool) -> str:
    status = "success" if success else "failed"
    stderr = "" if success else "\nstderr:\nfailed"
    return f"RESULT {call_id}:\nstatus: {status}\nstdout:\nok{stderr}"


def call(tool: str, call_id: str, **params: str) -> str:
    args = ", ".join([f'id="{call_id}"', *[f'{k}="{v}"' for k, v in params.items()]])
    return f"CALL {tool}({args})"


def raw_trace(assistant: str, results: list[str]) -> str:
    by_id = {}
    for block in results:
        first = block.splitlines()[0]
        if first.startswith("RESULT ") and first.endswith(":"):
            by_id[first.removeprefix("RESULT ").removesuffix(":")] = block

    parts = []
    for line in assistant.splitlines():
        parts.append(line)
        parsed = parse_calls(line)
        if parsed and parsed[0].id in by_id:
            parts.append(by_id[parsed[0].id])
    return "\n".join(parts)


def score(assistant: str, results: list[str], expected_tool: str = "read_file") -> float:
    calls = [
        {"tool": c.tool, "id": c.id, "params": c.params}
        for c in parse_calls(assistant)
    ]
    return asyncio.run(
        _rust_tool_reward(
            [{"role": "assistant", "content": assistant}],
            state={
                "executed_tool_calls": calls,
                "executed_result_blocks": results,
                "raw_chatml_transcript": raw_trace(assistant, results),
            },
            info={
                "expected_tool": expected_tool,
                "expected_args": {"file_path": "src/lib.rs"} if expected_tool else {},
            },
        )
    )


class RewardGoldenTests(unittest.TestCase):
    def test_finalization_terms_are_additive(self) -> None:
        read = call("read_file", "c1", file_path="src/lib.rs")
        result = [result_block("c1", True)]

        no_final = score(read, result)
        clean_final = score("\n".join([read, "FINAL: fixed"]), result)
        dirty_final = score("\n".join([read, "FINAL: fixed", "extra text"]), result)
        double_final = score("\n".join([read, "FINAL: one", "FINAL: two"]), result)

        self.assertAlmostEqual(clean_final - no_final, 3.0)
        self.assertAlmostEqual(clean_final - dirty_final, 0.5)
        self.assertAlmostEqual(clean_final - double_final, 1.0)

    def test_verifier_success_does_not_add_ladder_reward(self) -> None:
        read = call("read_file", "c1", file_path="src/lib.rs")
        patch = call("apply_patch", "c2", file_path="src/lib.rs", find="bug", replace="fix")
        ok = call("cargo_test", "c3", project_path=".")

        patched_no_verifier = score(
            "\n".join([read, patch]),
            [result_block("c1", True), result_block("c2", True)],
        )
        verifier_no_final = score(
            "\n".join([read, patch, ok]),
            [result_block("c1", True), result_block("c2", True), result_block("c3", True)],
        )

        self.assertAlmostEqual(patched_no_verifier, verifier_no_final)


def assistant_text(row: dict) -> str:
    return "\n".join(
        str(m.get("content") or "")
        for m in row.get("completion", [])
        if isinstance(m, dict) and m.get("role") == "assistant"
    )


def tool_text(row: dict) -> str:
    return "\n".join(
        str(m.get("content") or "")
        for m in row.get("completion", [])
        if isinstance(m, dict) and m.get("role") == "tool"
    )


def trajectory(row: dict) -> tuple[list[str], str, bool, bool, bool]:
    atext = assistant_text(row)
    ttext = tool_text(row)
    calls = parse_calls(atext)
    tools = [c.tool for c in calls]
    outcomes = _verifier_outcomes(
        [{"tool": c.tool, "id": c.id, "params": c.params} for c in calls],
        ttext,
    )
    has_verifier = bool(outcomes) or any(t in VERIFIERS for t in tools)
    has_patch = "apply_patch" in tools
    has_final = "FINAL:" in atext
    clean_final = ended_cleanly_after_final(atext)
    executed = {c.tool for c in calls if _find_result_for(c.id, ttext) is not None}
    if outcomes and outcomes[-1] and clean_final:
        stage = "clean_final"
    elif outcomes and outcomes[-1]:
        stage = "terminal_pass"
    elif outcomes:
        stage = "failed_terminal"
    elif "apply_patch" in executed:
        stage = "patch"
    elif "read_file" in executed:
        stage = "read"
    else:
        stage = "quit"
    return tools, stage, has_patch, has_verifier, has_final


def reward_bucket(value: float) -> str:
    if math.isnan(value):
        return "nan"
    if value < -4:
        return "<-4"
    if value < -2:
        return "[-4,-2)"
    if value < 0:
        return "[-2,0)"
    if value == 0:
        return "0"
    if value < 2:
        return "(0,2)"
    if value < 4:
        return "[2,4)"
    return ">=4"


def stream_rows(path: Path, limit: int):
    with path.open(encoding="utf-8") as handle:
        for idx, raw in enumerate(handle):
            if idx >= limit:
                break
            yield json.loads(raw)


def summarize_rollouts(paths: list[Path], sample: int) -> None:
    for path in paths:
        if not path.exists():
            print(f"\n{path}: missing")
            continue

        stage_counts: Counter[str] = Counter()
        reward_counts: Counter[str] = Counter()
        tool_counts: Counter[tuple[str, ...]] = Counter()
        pattern_counts: Counter[tuple[str, ...]] = Counter()
        examples: dict[tuple[str, ...], list[dict]] = defaultdict(list)
        rewards: list[float] = []
        patch = verifier = final = total = 0

        for row in stream_rows(path, sample):
            tools, stage, has_patch, has_verifier, has_final = trajectory(row)
            reward = float(row.get("reward", 0.0))
            pattern = tuple(tools) or ("<no_call>",)
            total += 1
            rewards.append(reward)
            stage_counts[stage] += 1
            reward_counts[reward_bucket(reward)] += 1
            tool_counts[tuple(tools)] += 1
            pattern_counts[pattern] += 1
            patch += has_patch
            verifier += has_verifier
            final += has_final
            if len(examples[pattern]) < 3:
                examples[pattern].append(
                    {"id": row.get("example_id"), "reward": round(reward, 3), "stage": stage}
                )

        print(f"\n{path} sample={total}")
        if not total:
            continue
        print(f"reward mean={statistics.mean(rewards):.3f} min={min(rewards):.3f} max={max(rewards):.3f}")
        print("stage distribution:", dict(stage_counts.most_common()))
        print("reward distribution:", dict(reward_counts.most_common()))
        print("tool sequence frequencies:", {str(k): v for k, v in tool_counts.most_common(10)})
        print(f"reaches apply_patch={patch / total:.1%} verifier={verifier / total:.1%} FINAL={final / total:.1%}")
        print("top trajectory patterns:")
        for pattern, count in pattern_counts.most_common(10):
            print(f"  {count:4d} {pattern} examples={examples[pattern]}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--sample", type=int, default=200, help="Rows to stream per rollout file.")
    parser.add_argument("--no-rollouts", action="store_true")
    args = parser.parse_args()

    suite = unittest.defaultTestLoader.loadTestsFromTestCase(RewardGoldenTests)
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    if not result.wasSuccessful():
        return 1
    if not args.no_rollouts:
        summarize_rollouts(ROLLOUT_PATHS, args.sample)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
