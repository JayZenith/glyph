from __future__ import annotations

import os
import shutil
import subprocess
from dataclasses import dataclass


@dataclass
class ExecutionResult:
    success: bool
    stdout: str
    stderr: str
    exit_code: int
    timed_out: bool = False
    sandboxed: bool = False


class RustExecutor:
    def __init__(
        self,
        nsjail_path: str = "nsjail",
        timeout: int = 30,
    ):
        self.nsjail_path = nsjail_path
        self.timeout = timeout

    def _resolve_command(self, command: list[str]) -> tuple[list[str], bool]:
        nsjail = shutil.which(self.nsjail_path)
        if nsjail:
            return (
                [
                    nsjail,
                    "-Mo",
                    "--config",
                    "/dev/null",
                    "--",
                    *command,
                ],
                True,
            )
        return (command, False)

    def execute(
        self,
        command: list[str],
        working_dir: str | None = None,
        env: dict[str, str] | None = None,
    ) -> ExecutionResult:
        nsjail_env = {
            "LANG": "en_US.UTF-8",
            "HOME": "/tmp",
            "TMPDIR": "/tmp",
        }
        if env:
            nsjail_env.update(env)

        run_cmd, sandboxed = self._resolve_command(command)

        try:
            result = subprocess.run(
                run_cmd,
                capture_output=True,
                text=True,
                timeout=self.timeout,
                cwd=working_dir,
                env=nsjail_env,
            )
            return ExecutionResult(
                success=result.returncode == 0,
                stdout=result.stdout,
                stderr=result.stderr,
                exit_code=result.returncode,
                sandboxed=sandboxed,
            )
        except subprocess.TimeoutExpired:
            return ExecutionResult(
                success=False,
                stdout="",
                stderr=f"Execution timed out after {self.timeout}s",
                exit_code=-1,
                timed_out=True,
                sandboxed=sandboxed,
            )
        except FileNotFoundError:
            return ExecutionResult(
                success=False,
                stdout="",
                stderr=f"command not found: {command[0]}",
                exit_code=-1,
            )

    def compile_file(self, file_path: str, output_path: str | None = None) -> ExecutionResult:
        cmd = ["rustc", file_path]
        if output_path:
            cmd.extend(["-o", output_path])
        return self.execute(cmd)

    def run_binary(self, binary_path: str) -> ExecutionResult:
        return self.execute([binary_path])

    def cargo_check(self, project_path: str) -> ExecutionResult:
        return self.execute(["cargo", "check"], working_dir=project_path)

    def cargo_test(self, project_path: str, test_name: str | None = None) -> ExecutionResult:
        cmd = ["cargo", "test"]
        if test_name:
            cmd.extend(["--", test_name])
        return self.execute(cmd, working_dir=project_path)

    def cargo_build(self, project_path: str, release: bool = False) -> ExecutionResult:
        cmd = ["cargo", "build"]
        if release:
            cmd.append("--release")
        return self.execute(cmd, working_dir=project_path)


def create_executor(
    nsjail_path: str | None = None,
    timeout: int = 30,
) -> RustExecutor:
    return RustExecutor(
        nsjail_path=nsjail_path or os.environ.get("NSJAIL_PATH", "nsjail"),
        timeout=timeout,
    )
