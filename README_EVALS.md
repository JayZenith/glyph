# Eval Notes

This repo has two eval styles that answer different questions:

- **HF/Transformers formal eval** (`sft.eval_formal`): one rollout per prompt, scored as a valid trace only if the model reaches a successful cargo verifier and ends cleanly with `FINAL`.
- **vLLM pass@k scan** (`sft/passk_scan_vllm.py`): `k` sampled rollouts per prompt. Current pass@k artifacts use the `terminal_tool_success` metric from `score_output`; this is a terminal tool-status metric, not a guaranteed cargo-verifier-success metric.

## Failure Buckets

Buckets are not mutually exclusive: one trace can be `missing_final`, `dirty_final`, and `final_before_tool_completion` at the same time.

| eval | valid traces | `terminal_tool_success` metric | actual cargo successes in trace | clean end | dirty/final/missing | task failure bucket | truncated | repetition |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `SFT_V1` HF formal heldout-69 | 52/69 | 68/69 | 52/69 | 0.754 | 17 | 1 | 0 | 0 |
| `RLVR_V1` HF formal heldout-69 | 20/69 | 48/69 | 20/69 | 0.333 | 46 | 21 | 5 | 0 |
| `RLVR_B` HF formal heldout-69 | 19/69 | 46/69 | 19/69 | 0.275 | 50 | 23 | 7 | 3 |
| `RLVR_HELDOUT69_PASSK_STEP25` HF formal heldout-69 | 19/69 | 47/69 | 19/69 | 0.304 | 48 | 22 | 7 | 0 |

Important caveat: `terminal_tool_success` is currently computed from the last parsed tool
call in `score_output`. If the last tool is `read_file` or `apply_patch` and it succeeds,
the metric can be true even though no `cargo_run` or `cargo_test` passed. Manual inspection
of the 17 invalid SFT_V1 formal traces found **0/17 actual cargo successes**; 16/17 had
`terminal_tool_success=True` only because they ended after a successful non-verifier tool.

Primary artifacts:

```text
results/SFT_V1/eval_formal_heldout_69.json
results/RLVR_V1/eval_formal_heldout_69.json
results/RLVR_B/heldout_69/eval_heldout_69.json
results/RLVR_HELDOUT69_FORMAL_STEP25/eval_formal_heldout_69_step25.json
```

## Sample Rollout

One clean SFT_V1 recovery trace from `eval100_024_patch_run_pass_005_active_tags_filter_map_join`:

```text
CALL read_file(id="c1", file_path=".../src/main.rs")
RESULT c1: status: success
stdout:
    .filter_map(|(name, enabled)| if !enabled { Some(*name) } else { None })

CALL apply_patch(id="c2", ..., replace=".filter_map(|(name, enabled)| if enabled { Some(*name) } else { None })")
RESULT c2: status: success
stdout: patch applied

CALL cargo_run(id="c3", project_path="...")
RESULT c3: status: failed
stderr:
error[E0308]: mismatched types
help: consider dereferencing the borrow

CALL read_file(id="c4", file_path=".../src/main.rs")
RESULT c4: status: success

CALL apply_patch(id="c5", ..., replace=".filter_map(|(name, enabled)| if *enabled { Some(*name) } else { None })")
RESULT c5: status: success
stdout: patch applied

CALL cargo_run(id="c6", project_path="...")
RESULT c6: status: success
stdout: core,fast,safe

FINAL: Fixed the iterator filter_map condition so it keeps active tags and dereferences the boolean flag, producing the expected stdout.
```

## pass@k Table

All pass@k rows below use vLLM sampling at `k=8`, `T=0.8`.

| scan | prompts | solve histogram | total solves | mean pass@8 | use |
|---|---:|---|---:|---:|---|
| train depth>=3 SFT_V1 scan | 134 | `{4:1, 5:4, 6:7, 7:27, 8:95}` | 1015/1072 | 0.947 | found 39 train rlvr-target prompts |
| original SFT_V1 formal failures rescanned | 17 | `{1:1, 6:3, 7:4, 8:9}` | 119/136 | 0.875 | reported many terminal-tool-metric successes; rollouts need cargo-success audit |
| final 8 heldout-failure targets, SFT_V1 | 8 | selected mixed band | 47/64 | 0.734 | before for narrow RLVR run |
| final 8 heldout-failure targets, step 25 | 8 | selected mixed band | 54/64 | 0.844 | narrow matched vLLM win |

Primary artifacts:

```text
synthetic_data/passk_train134.json
results/SFT_V1/passk_heldout69_fail17.json
results/RLVR_HELDOUT69_PASSK_STEP25/passk8_step25_comparison.csv
```

## HF vs vLLM Harness Mismatch

The heldout-69 "failures" were not all the same kind of failure. The original HF/Transformers formal eval required a clean valid trace. Manual inspection of the 17 invalid SFT_V1 traces showed they did **not** actually reach a successful cargo verifier; many repeatedly failed `cargo_run`/`cargo_test`, then ended after a successful non-verifier tool such as `read_file`, which inflated the `terminal_tool_success` metric. The later vLLM pass@8 scan used the same prompts/cases and the same CALL/RESULT tool protocol, but sampled multiple rollouts and counted the same terminal-tool metric for banding. So the mismatch was not "protocol eval versus no protocol eval." It was single-sample valid-trace scoring versus sampled terminal-tool-status banding, and the pass@k artifacts should be read with that scoring caveat.
