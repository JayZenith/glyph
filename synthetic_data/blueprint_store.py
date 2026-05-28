#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import shutil
from pathlib import Path


def case_ids(data: Path) -> list[str]:
    ids: list[str] = []
    seen: set[str] = set()
    with data.open(encoding="utf-8") as handle:
        for line_no, raw in enumerate(handle, 1):
            raw = raw.strip()
            if not raw:
                continue
            row = json.loads(raw)
            case_id = row.get("case_id")
            if not isinstance(case_id, str):
                raise SystemExit(f"{data}:{line_no}: missing string case_id")
            if case_id not in seen:
                ids.append(case_id)
                seen.add(case_id)
    return ids


def copy_tree(src: Path, dst: Path, overwrite: bool) -> None:
    if not src.exists():
        raise FileNotFoundError(str(src))
    if dst.exists():
        if not overwrite:
            return
        shutil.rmtree(dst)
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copytree(
        src,
        dst,
        ignore=shutil.ignore_patterns("target", ".git", ".cargo"),
    )


def run_export(ids: list[str], source_root: Path, store_root: Path, overwrite: bool) -> tuple[int, list[str]]:
    missing: list[str] = []
    copied = 0
    for case_id in ids:
        src = source_root / case_id
        dst = store_root / case_id
        try:
            before = dst.exists()
            copy_tree(src, dst, overwrite)
            copied += int(overwrite or not before)
        except FileNotFoundError:
            missing.append(case_id)
    return copied, missing


def run_restore(ids: list[str], source_root: Path, store_root: Path, overwrite: bool) -> tuple[int, list[str]]:
    missing: list[str] = []
    copied = 0
    for case_id in ids:
        src = store_root / case_id
        dst = source_root / case_id
        try:
            before = dst.exists()
            copy_tree(src, dst, overwrite)
            copied += int(overwrite or not before)
        except FileNotFoundError:
            missing.append(case_id)
    return copied, missing


def main() -> int:
    parser = argparse.ArgumentParser(description="Export/restore durable Rust blueprint crates for a dataset.")
    parser.add_argument("cmd", choices=["export", "restore"])
    parser.add_argument("--data", type=Path, required=True)
    parser.add_argument("--source-root", type=Path, default=Path("runs/rlvr1/rust_cases"))
    parser.add_argument("--store-root", type=Path, default=Path("synthetic_data/blueprints"))
    parser.add_argument("--overwrite", action="store_true")
    args = parser.parse_args()

    ids = case_ids(args.data)
    if args.cmd == "export":
        copied, missing = run_export(ids, args.source_root, args.store_root, args.overwrite)
    else:
        copied, missing = run_restore(ids, args.source_root, args.store_root, args.overwrite)

    print(json.dumps({
        "cmd": args.cmd,
        "data": str(args.data),
        "case_ids": len(ids),
        "copied": copied,
        "missing": len(missing),
        "missing_sample": missing[:20],
    }, indent=2))
    return 1 if missing else 0


if __name__ == "__main__":
    raise SystemExit(main())
