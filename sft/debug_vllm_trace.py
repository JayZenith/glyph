#!/usr/bin/env python3
"""Dump one vLLM rollout trace for the first rlvr-target prompt to debug 0/8."""
from __future__ import annotations

import sys
from pathlib import Path

from agent_runtime.chatml import render_tool_turn
from sft.evals import build_prompt, load_prompts, score_output
from sft.eval_formal import prepare_eval_items
from agent_runtime.protocol import assistant_text, extract_pending_call_ids, tool_text
from agent_runtime.rust.executor import create_executor
from agent_runtime.rust.results import format_result_block, parse_call_blocks
from agent_runtime.rust.runtime import ensure_sandbox_copy, execute_rust_tool, rewrite_params_for_sandbox


def main() -> None:
    MODEL = sys.argv[1] if len(sys.argv) > 1 else "outputs/rlvr_passk/weights/step_25"
    prompts = load_prompts("rlvr_target_39", "runs/rlvr_passk_train150/prompts.yaml")
    prompts = prepare_eval_items(prompts, Path("runs/rlvr_passk_train150/cases"))
    item = prompts[0]
    print("ITEM:", item["name"])
    print("blueprint_root:", item.get("blueprint_root"))
    print("trace_prefix:", item.get("trace_prefix"))
    print("expected_output:", repr(item.get("expected_output"))[:200])

    from transformers import AutoTokenizer
    from vllm import LLM, SamplingParams

    tok = AutoTokenizer.from_pretrained(MODEL, trust_remote_code=True)
    llm = LLM(model=MODEL, dtype="bfloat16", gpu_memory_utilization=0.85,
              max_model_len=12288, trust_remote_code=True, enforce_eager=False)
    executor = create_executor(timeout=30)

    prompt = build_prompt(item["user"], item.get("system"))
    print("\n===== PROMPT (tail) =====\n", prompt[-600:])
    blueprint_root = item.get("blueprint_root")
    trace_prefix = item.get("trace_prefix") or blueprint_root
    _, sandbox = ensure_sandbox_copy(blueprint_root, Path("runs/rlvr_passk_train150/cases/_dbg"))
    print("sandbox:", sandbox)

    acc = ""
    remaining = 4000
    cur = prompt
    for rnd in range(16):
        if remaining <= 0:
            print("OUT OF TOKENS"); break
        stop_ids = [tok.eos_token_id, tok.convert_tokens_to_ids("<|im_end|>"),
                    tok.convert_tokens_to_ids("<|im_start|>")]
        sp = SamplingParams(n=1, temperature=0.0, top_p=1.0, max_tokens=remaining,
                            stop_token_ids=stop_ids)
        ids = tok(cur, add_special_tokens=False)["input_ids"]
        out = llm.generate([{"prompt_token_ids": ids}], [sp], use_tqdm=False)[0].outputs[0]
        text = out.text + ("<|im_end|>" if out.finish_reason == "stop" else "")
        acc += text
        remaining -= len(out.token_ids)
        print(f"\n----- ROUND {rnd} gen ({len(out.token_ids)} tok, finish={out.finish_reason}) -----")
        print(repr(out.text)[:1000])
        full = prompt + acc
        pend = extract_pending_call_ids(assistant_text(full), tool_text(full))
        print("pending:", pend)
        if not pend:
            print("NO PENDING -> done"); break
        injected = ""
        for call in parse_call_blocks(assistant_text(full)):
            if call["id"] not in pend:
                continue
            params = call["params"]
            if trace_prefix and sandbox:
                params = rewrite_params_for_sandbox(params, trace_prefix, sandbox)
            res = execute_rust_tool(executor, call["tool"], params,
                                    expected_output=item.get("expected_output") if call["tool"] == "cargo_run" else None)
            print(f"  EXEC {call['tool']} {params} -> success={res.success} stdout={repr(res.stdout)[:120]} stderr={repr(res.stderr)[:150]}")
            injected += render_tool_turn(format_result_block(call["id"], res))
        acc += injected
        cur = prompt + acc

    m = score_output(prompt, acc.strip(), item, 0, 4000)
    print("\n===== SCORE =====")
    print("terminal_tool_success:", m["terminal_tool_success"], "call_sequence:", m.get("call_sequence"))


if __name__ == "__main__":
    main()
