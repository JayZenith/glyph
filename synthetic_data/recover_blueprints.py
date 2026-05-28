#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from agent_runtime.rust.results import parse_call_blocks

SEG_RE = re.compile(r"<\|im_start\|>(\w+)\n(.*?)<\|im_end\|>", re.DOTALL)
RESULT_RE = re.compile(r"RESULT\s+([A-Za-z0-9_\-]+):\n(.*)", re.DOTALL)
STDOUT_RE = re.compile(r"stdout:\n(.*?)(?:\nstderr:|\Z)", re.DOTALL)
PATH_RE = re.compile(r"runs/rlvr1/rust_cases/([^/]+)/(.+)$")


def segments(trace: str) -> list[tuple[str, str]]:
    return SEG_RE.findall(trace)


def stdout_from_tool(body: str) -> str | None:
    match = RESULT_RE.search(body.strip())
    if not match:
        return None
    stdout = STDOUT_RE.search(match.group(2).strip())
    return stdout.group(1).rstrip("\n") + "\n" if stdout else None


def cargo_toml(case_name: str, has_main: bool) -> str:
    package = re.sub(r"[^a-zA-Z0-9_-]+", "_", case_name)
    return f'[package]\nname = "{package}"\nversion = "0.1.0"\nedition = "2021"\n'


def recover_row(row: dict, source_root: Path, overwrite: bool) -> str | None:
    trace = row.get("trace")
    case_id = row.get("case_id")
    if not isinstance(trace, str) or not isinstance(case_id, str):
        return "missing trace or case_id"

    calls = parse_call_blocks("\n".join(body for role, body in segments(trace) if role == "assistant"))
    tool_bodies = [body for role, body in segments(trace) if role == "tool"]
    project = source_root / case_id
    if project.exists() and not overwrite:
        return None

    wrote_source = False
    for call, tool_body in zip(calls, tool_bodies):
        if call["tool"] != "read_file":
            continue
        file_path = call["params"].get("file_path")
        if not isinstance(file_path, str):
            continue
        match = PATH_RE.match(file_path)
        if not match or match.group(1) != case_id:
            continue
        rel = match.group(2)
        if rel not in {"src/lib.rs", "src/main.rs"}:
            continue
        stdout = stdout_from_tool(tool_body)
        if stdout is None:
            continue
        target = project / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(stdout, encoding="utf-8")
        wrote_source = True
        break

    if not wrote_source:
        return "no read_file source recovered"

    has_main = (project / "src" / "main.rs").exists()
    (project / "Cargo.toml").write_text(cargo_toml(case_id, has_main), encoding="utf-8")
    return None


def main() -> int:
    parser = argparse.ArgumentParser(description="Recover source-root blueprint crates from trace read_file outputs.")
    parser.add_argument("data", type=Path)
    parser.add_argument("--source-root", type=Path, default=Path("runs/rlvr1/rust_cases"))
    parser.add_argument("--overwrite", action="store_true")
    args = parser.parse_args()

    errors: list[str] = []
    recovered = 0
    for line_no, raw in enumerate(args.data.read_text(encoding="utf-8").splitlines(), 1):
        if not raw.strip():
            continue
        row = json.loads(raw)
        err = recover_row(row, args.source_root, args.overwrite)
        if err:
            errors.append(f"line {line_no} {row.get('case_id')}: {err}")
        else:
            recovered += 1

    print(json.dumps({"recovered_or_present": recovered, "errors": len(errors)}, indent=2))
    if errors:
        for error in errors[:50]:
            print(error)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
