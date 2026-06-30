"""Smoke tests for the dense / compiler-aware progress rewards.

Confirms each rustc compiler-phase ladder stage receives exactly the expected
partial credit, that the ladder is monotone (later phase outranks earlier even
with more errors), and that the generic dense arm and the default sparse reward
are unchanged. Runs standalone (`python3 rl/tests/test_reward_progress.py`) or
under pytest.
"""
from __future__ import annotations

import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))

from agent_runtime.protocol import ProtocolCall
from agent_runtime.rust.executor import ExecutionResult
from rl.reward import _progress_reward, build_reward_config

LADDER = 2.5  # --progress-error-ladder-bonus used in these checks

# Representative cargo stderr for each compiler phase, furthest reached last.
STAGE_OUTPUT = {
    0: "warning: unused import",                         # no cargo error seen
    1: "error: expected one of `;`, found `}`",          # parse / syntax
    2: "error[E0308]: mismatched types",                 # type / resolution
    3: "error[E0502]: cannot borrow `x` as mutable",     # borrow / lifetime
}


def _call(cid: str = "c1") -> ProtocolCall:
    return ProtocolCall(tool="cargo_test", id=cid, params={"project_path": "."})


def _result(stderr: str, success: bool = False) -> ExecutionResult:
    return ExecutionResult(success=success, stdout="", stderr=stderr, exit_code=0 if success else 1)


def _ladder_only(**over):
    cfg = {"progress_compile_bonus": 0.0, "progress_test_frac_bonus": 0.0,
           "progress_error_ladder_bonus": LADDER}
    cfg.update(over)
    return build_reward_config(cfg)


def test_each_ladder_stage_reward():
    cfg = _ladder_only()
    for stage, out in STAGE_OUTPUT.items():
        got = _progress_reward([_call()], {"c1": _result(out)}, cfg)
        want = LADDER * stage / 4.0
        assert abs(got - want) < 1e-9, f"stage {stage}: got {got}, want {want}"


def test_compiled_stage_is_top():
    cfg = _ladder_only()
    compiled = _result("test result: FAILED. 0 passed; 3 failed", success=False)
    got = _progress_reward([_call()], {"c1": compiled}, cfg)
    assert abs(got - LADDER) < 1e-9, got  # stage 4 -> full ladder bonus


def test_ladder_is_monotone_not_error_count():
    # Borrow phase with ONE error must outscore type phase with MANY errors:
    # progress is phase reached, not error count -> not gameable by churning.
    cfg = _ladder_only()
    borrow_one = _progress_reward([_call()], {"c1": _result("error[E0502]: x")}, cfg)
    type_many = _progress_reward(
        [_call()],
        {"c1": _result("error[E0308]\nerror[E0277]\nerror[E0425]\nerror[E0412]")},
        cfg,
    )
    assert borrow_one > type_many, (borrow_one, type_many)


def test_best_stage_across_rollout():
    # Two cargo calls: an early parse error then a later borrow error -> stage 3.
    cfg = _ladder_only()
    calls = [_call("c1"), _call("c2")]
    results = {"c1": _result(STAGE_OUTPUT[1]), "c2": _result(STAGE_OUTPUT[3])}
    got = _progress_reward(calls, results, cfg)
    assert abs(got - LADDER * 3 / 4.0) < 1e-9, got


def test_generic_dense_arm_unchanged():
    cfg = build_reward_config({"progress_compile_bonus": 0.5,
                               "progress_test_frac_bonus": 2.0})
    compiled_half = _result("test result: FAILED. 2 passed; 2 failed")
    got = _progress_reward([_call()], {"c1": compiled_half}, cfg)
    assert abs(got - (0.5 + 2.0 * 0.5)) < 1e-9, got  # compile bonus + half tests


def test_sparse_default_is_zero():
    cfg = build_reward_config({})  # all progress bonuses default 0.0
    got = _progress_reward([_call()], {"c1": _result(STAGE_OUTPUT[3])}, cfg)
    assert got == 0.0, got


if __name__ == "__main__":
    fns = [v for k, v in sorted(globals().items()) if k.startswith("test_")]
    for fn in fns:
        fn()
        print(f"PASS {fn.__name__}")
    print(f"\n{len(fns)} passed")
