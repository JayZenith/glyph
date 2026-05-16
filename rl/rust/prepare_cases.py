#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path

from sft.evals.prompt_loader import build_prompt
from rl.rust.tools import RUST_TOOLS as _RUST_TOOLS


# build_prompt expects list[dict]; convert from the canonical Tool dataclass list.
RUST_TOOLS: list[dict] = [
    {"name": t.name, "description": t.description, "params": t.params}
    for t in _RUST_TOOLS
]


def write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def cargo_toml(name: str) -> str:
    return f"""[package]
name = "{name}"
version = "0.1.0"
edition = "2021"
"""


def build_cases(root: Path) -> list[dict]:
    cases: list[dict] = []

    fiblib = root / "fiblib"
    write(fiblib / "Cargo.toml", cargo_toml("fiblib"))
    write(
        fiblib / "src" / "lib.rs",
        """pub fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::fib;

    #[test]
    fn fib_small() {
        assert_eq!(fib(7), 13);
    }
}
""",
    )
    cases.append({
        "task": f'Verify whether the Cargo project at "{fiblib}" passes its tests.',
        "expected_tool": "cargo_test",
        "expected_args": {"project_path": str(fiblib)},
    })

    sortlib = root / "sortlib"
    write(sortlib / "Cargo.toml", cargo_toml("sortlib"))
    write(
        sortlib / "src" / "lib.rs",
        """pub fn sort_numbers(values: &mut [i32]) {
    values.sort();
}

#[cfg(test)]
mod tests {
    use super::sort_numbers;

    #[test]
    fn sorts_values() {
        let mut values = [3, 1, 2];
        sort_numbers(&mut values);
        assert_eq!(values, [1, 2, 3]);
    }
}
""",
    )
    cases.append({
        "task": f'Run the correct verification call to confirm that the Cargo project at "{sortlib}" passes its tests.',
        "expected_tool": "cargo_test",
        "expected_args": {"project_path": str(sortlib)},
    })

    mathlib = root / "mathlib"
    write(mathlib / "Cargo.toml", cargo_toml("mathlib"))
    write(
        mathlib / "src" / "lib.rs",
        """pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
""",
    )
    cases.append({
        "task": f'Check whether the Rust project at "{mathlib}" compiles cleanly without producing a binary.',
        "expected_tool": "cargo_check",
        "expected_args": {"project_path": str(mathlib)},
    })

    parserlib = root / "parserlib"
    write(parserlib / "Cargo.toml", cargo_toml("parserlib"))
    write(
        parserlib / "src" / "lib.rs",
        """pub fn first_word(input: &str) -> &str {
    input.split_whitespace().next().unwrap_or("")
}
""",
    )
    cases.append({
        "task": f'Issue the compile-only verification call for the Cargo project at "{parserlib}".',
        "expected_tool": "cargo_check",
        "expected_args": {"project_path": str(parserlib)},
    })

    greeter = root / "greeter"
    write(greeter / "Cargo.toml", cargo_toml("greeter"))
    write(
        greeter / "src" / "main.rs",
        """fn main() {
    println!("hello from greeter");
}
""",
    )
    cases.append({
        "task": f'Build the Cargo binary project at "{greeter}" using the correct Rust build tool.',
        "expected_tool": "cargo_build",
        "expected_args": {"project_path": str(greeter)},
    })

    counter = root / "counter"
    write(counter / "Cargo.toml", cargo_toml("counter"))
    write(
        counter / "src" / "main.rs",
        """fn main() {
    for value in 1..=3 {
        println!("{value}");
    }
}
""",
    )
    cases.append({
        "task": f'Compile the Cargo app at "{counter}" into a binary using the right build tool.',
        "expected_tool": "cargo_build",
        "expected_args": {"project_path": str(counter)},
    })

    hello_file = root / "hello.rs"
    hello_bin = root / "hello_bin"
    write(
        hello_file,
        """fn main() {
    println!("hello");
}
""",
    )
    cases.append({
        "task": f'Compile the single Rust source file at "{hello_file}" to the binary "{hello_bin}".',
        "expected_tool": "rustc",
        "expected_args": {
            "source_file": str(hello_file),
            "output": str(hello_bin),
        },
    })

    sum_file = root / "sum.rs"
    sum_bin = root / "sum_bin"
    write(
        sum_file,
        """fn main() {
    let values = [1, 2, 3, 4];
    let total: i32 = values.iter().sum();
    println!("{total}");
}
""",
    )
    cases.append({
        "task": f'Use rustc to compile "{sum_file}" to the binary "{sum_bin}".',
        "expected_tool": "rustc",
        "expected_args": {
            "source_file": str(sum_file),
            "output": str(sum_bin),
        },
    })

    return cases


def prompt_for(task: str) -> str:
    return build_prompt(task, RUST_TOOLS)


def main() -> None:
    parser = argparse.ArgumentParser(description="Prepare Rust tool RL cases and prompt JSONL.")
    parser.add_argument("--root", type=Path, default=Path("runs/rl1/rust_tool_cases"))
    parser.add_argument("--output", type=Path, default=Path("runs/rl1/rust_tool_prompts_8.jsonl"))
    args = parser.parse_args()

    args.root.mkdir(parents=True, exist_ok=True)
    cases = build_cases(args.root.resolve())

    args.output.parent.mkdir(parents=True, exist_ok=True)
    with args.output.open("w", encoding="utf-8") as f:
        for case in cases:
            row = {
                "prompt": prompt_for(case["task"]),
                "expected_tool": case["expected_tool"],
                "expected_args": case["expected_args"],
            }
            f.write(json.dumps(row, ensure_ascii=False) + "\n")

    print(f"Wrote {len(cases)} prompts to {args.output}")
    print(f"Wrote Rust cases under {args.root}")


if __name__ == "__main__":
    main()
