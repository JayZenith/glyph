from __future__ import annotations

from dataclasses import dataclass


@dataclass
class ToolReward:
    total: float
    components: dict[str, float]
    details: dict


def compute_tool_reward(
    tool_name: str,
    execution_result: dict,
    expected_output: str | None = None,
) -> ToolReward:
    success = execution_result.get("success", False)
    stdout = execution_result.get("stdout", "")
    stderr = execution_result.get("stderr", "")
    exit_code = execution_result.get("exit_code", -1)
    timed_out = execution_result.get("timed_out", False)

    components = {}
    details = {}

    if timed_out:
        components["timeout"] = -1.0
        details["reason"] = "execution timed out"
        return ToolReward(total=-1.0, components=components, details=details)

    if tool_name == "rustc":
        if success:
            components["compilation_success"] = 1.0
            if stderr and "warning" in stderr.lower():
                components["compilation_warnings"] = -0.1
            else:
                components["compilation_warnings"] = 0.0
        else:
            components["compilation_success"] = -1.0
            details["error_snippet"] = stderr[:500] if stderr else "unknown error"

    elif tool_name == "cargo_check":
        if success:
            components["check_success"] = 1.0
            if stderr and "warning" in stderr.lower():
                components["check_warnings"] = -0.1
            else:
                components["check_warnings"] = 0.0
        else:
            components["check_success"] = -1.0
            details["error_snippet"] = stderr[:500] if stderr else "unknown error"

    elif tool_name == "cargo_build":
        if success:
            components["build_success"] = 1.0
            if stderr and "warning" in stderr.lower():
                components["build_warnings"] = -0.1
            else:
                components["build_warnings"] = 0.0
        else:
            components["build_success"] = -1.0
            details["error_snippet"] = stderr[:500] if stderr else "unknown error"

    elif tool_name == "cargo_test":
        if success:
            components["test_pass"] = 1.0

            if "test result: ok" in stdout.lower():
                components["all_tests_pass"] = 0.5

            test_count = stdout.count("test result:")
            if test_count > 0:
                details["test_runs"] = test_count

            if "FAILED" in stdout or "failed" in stdout:
                components["test_pass"] = -0.5
                components["all_tests_pass"] = -0.3
        else:
            components["test_pass"] = -1.0
            details["error_snippet"] = stderr[:500] if stderr else "unknown error"

    elif tool_name == "execute":
        if success:
            components["execution_success"] = 1.0
            details["exit_code"] = exit_code

            if expected_output is not None:
                if stdout.strip() == expected_output.strip():
                    components["output_match"] = 1.0
                else:
                    components["output_match"] = -0.5
                    details["expected"] = expected_output[:200]
                    details["actual"] = stdout[:200]
        else:
            components["execution_success"] = -1.0
            details["error_snippet"] = stderr[:500] if stderr else "unknown error"

    else:
        components["unknown_tool"] = -0.5

    total = sum(components.values())
    return ToolReward(total=total, components=components, details=details)
