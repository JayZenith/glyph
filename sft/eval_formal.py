#!/usr/bin/env python3
"""Format-quality eval. Loads SFT (and optionally base), generates on each
prompt, scores with the validator, writes JSON.

Run: python -m sft.eval_formal --output ... [--include-base] [--limit N]
"""
import argparse
import json
import subprocess
from datetime import datetime, timezone
from pathlib import Path

from sft.evals import (
    build_prompt,
    generate,
    load_model,
    load_prompts,
    score_output,
    summarize,
)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base-model", default="Qwen/Qwen3-4B-Base")
    parser.add_argument("--sft-model", default="JayZenith/GLYPH_SFT")
    parser.add_argument("--prompt-section", default="formal_eval_rl")
    parser.add_argument("--prompt-file", default=None,
                        help="Optional yaml file to load prompts from instead of sft/evals/prompts_125.yaml")
    parser.add_argument("--output", required=True)
    parser.add_argument("--max-new-tokens", type=int, default=6000)
    parser.add_argument("--max-tool-rounds", type=int, default=8,
                        help="Max rounds of mocked-tool-result injection per prompt")
    parser.add_argument("--limit", type=int, default=None,
                        help="Limit to first N prompts (for smoke runs)")
    parser.add_argument("--include-base", action="store_true",
                        help="Also evaluate the base model (default off; base output is fixed across ablations)")
    parser.add_argument("--stream-output", action="store_true",
                        help="Print each completed model output as soon as that prompt finishes")
    parser.add_argument("--stream-output-chars", type=int, default=0,
                        help="If > 0, truncate streamed output to this many chars")
    parser.add_argument("--token-stream", action="store_true",
                        help="Print generated text token-by-token as each prompt runs")
    args = parser.parse_args()

    if args.include_base:
        print("Loading base model...")
        base_model, base_tok = load_model(args.base_model)
    print("Loading SFT model...")
    sft_model, sft_tok = load_model(args.sft_model)

    prompts = load_prompts(args.prompt_section, args.prompt_file)
    if args.limit is not None:
        prompts = prompts[:args.limit]
    results = {"sft": []}
    if args.include_base:
        results["base"] = []
    for item in prompts:
        prompt = build_prompt(item["user"], item.get("tools", []), item.get("system"))
        tools = item.get("tools", [])

        def make_token_callback(label: str):
            if not args.token_stream:
                return None
            started = False

            def _cb(piece: str) -> None:
                nonlocal started
                if not started:
                    print(f"\n===== {item['name']} | {label} =====\n", end="", flush=True)
                    started = True
                print(piece, end="", flush=True)

            return _cb

        if args.include_base:
            print(f"Running {item['name']} on base...")
            base_out, base_n = generate(
                base_model,
                base_tok,
                prompt,
                args.max_new_tokens,
                max_tool_rounds=args.max_tool_rounds,
                token_callback=make_token_callback("base"),
            )
            if args.token_stream:
                print("\n", flush=True)
            if args.stream_output:
                shown = base_out if args.stream_output_chars <= 0 else base_out[:args.stream_output_chars]
                print(f"\n===== {item['name']} | base =====\n{shown}\n", flush=True)
            results["base"].append({
                "name": item["name"],
                "prompt": item["user"],
                "output": base_out,
                "metrics": score_output(prompt, base_out, item, base_n, args.max_new_tokens),
            })

        print(f"Running {item['name']} on sft...")
        sft_out, sft_n = generate(
            sft_model,
            sft_tok,
            prompt,
            args.max_new_tokens,
            max_tool_rounds=args.max_tool_rounds,
            token_callback=make_token_callback("sft"),
        )
        if args.token_stream:
            print("\n", flush=True)
        if args.stream_output:
            shown = sft_out if args.stream_output_chars <= 0 else sft_out[:args.stream_output_chars]
            print(f"\n===== {item['name']} | sft =====\n{shown}\n", flush=True)
        results["sft"].append({
            "name": item["name"],
            "prompt": item["user"],
            "output": sft_out,
            "metrics": score_output(prompt, sft_out, item, sft_n, args.max_new_tokens),
        })

    try:
        commit = subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip()
    except Exception:
        commit = None

    summary = {"sft": summarize("sft", results["sft"])}
    if args.include_base:
        summary["base"] = summarize("base", results["base"])

    payload = {
        "run": {
            "timestamp_utc": datetime.now(timezone.utc).isoformat(),
            "git_commit": commit,
            "args": vars(args),
            "n_prompts": len(prompts),
        },
        "summary": summary,
        "results": results,
    }
    Path(args.output).write_text(json.dumps(payload, indent=2))
    print(f"Wrote {args.output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
