#!/usr/bin/env python3
"""Format-quality eval for the SFT model only."""
import argparse
import json
import subprocess
from datetime import datetime, timezone
from pathlib import Path

from sft.evals import (
    assert_no_prompt_overlap,
    build_prompt,
    generate,
    load_model,
    load_prompts,
    score_output,
    summarize,
)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--sft-model", required=True)
    parser.add_argument("--prompt-section", default="post_eval")
    parser.add_argument("--prompt-file", default=None,
                        help="Optional yaml file to load prompts from instead of sft/evals/eval_prompts.yaml")
    parser.add_argument("--train-data", default=None,
                        help="Optional train dataset JSONL to check for exact prompt overlap against the eval set")
    parser.add_argument("--output", required=True)
    parser.add_argument("--max-new-tokens", type=int, default=6000)
    parser.add_argument("--max-tool-rounds", type=int, default=8,
                        help="Max rounds of mocked-tool-result injection per prompt")
    parser.add_argument("--limit", type=int, default=None,
                        help="Limit to first N prompts (for smoke runs)")
    parser.add_argument("--stream-output", action="store_true",
                        help="Print each completed model output as soon as that prompt finishes")
    parser.add_argument("--stream-output-chars", type=int, default=0,
                        help="If > 0, truncate streamed output to this many chars")
    parser.add_argument("--token-stream", action="store_true",
                        help="Print generated text token-by-token as each prompt runs")
    args = parser.parse_args()

    print("Loading SFT model...")
    sft_model, sft_tok = load_model(args.sft_model)

    prompts = load_prompts(args.prompt_section, args.prompt_file)
    if args.train_data:
        assert_no_prompt_overlap(prompts, args.train_data)
    if args.limit is not None:
        prompts = prompts[:args.limit]
    results = {"sft": []}
    for item in prompts:
        prompt = build_prompt(item["user"], item.get("system"))

        def make_token_callback() -> None:
            if not args.token_stream:
                return None
            started = False

            def _cb(piece: str) -> None:
                nonlocal started
                if not started:
                    print(f"\n===== {item['name']} | sft =====\n", end="", flush=True)
                    started = True
                print(piece, end="", flush=True)

            return _cb

        print(f"Running {item['name']} on sft...")
        sft_out, sft_n = generate(
            sft_model,
            sft_tok,
            prompt,
            args.max_new_tokens,
            max_tool_rounds=args.max_tool_rounds,
            token_callback=make_token_callback(),
            mock_results=item.get("mock_results"),
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
