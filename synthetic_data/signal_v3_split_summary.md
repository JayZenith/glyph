# signal_v3 deterministic split summary

- source: `synthetic_data/signal_v3.jsonl`
- seed: `glyph-signal-v3-clean-sft-rlvr-split-v1`
- method: stratified by `family`, `difficulty`, expected tool-sequence length, and run/test verifier family; grouped by `case_id` to prevent duplicate trace leakage.
- leakage checks: case_id overlap `0`, trace overlap `0`

| split | rows | unique_case_ids |
|---|---:|---:|
| SFT_HALF_A | 1042 | 762 |
| RL_POOL_B | 1041 | 760 |
| RL_POOL_B_PROMPTS | 760 | 760 |

## Family
| bucket | SFT_HALF_A | RL_POOL_B |
|---|---:|---:|
| patch_run_pass | 194 | 200 |
| patch_run_recover | 150 | 148 |
| patch_test_pass | 366 | 362 |
| patch_test_recover | 280 | 281 |
| run_only | 16 | 15 |
| test_only | 36 | 35 |

## Difficulty
| bucket | SFT_HALF_A | RL_POOL_B |
|---|---:|---:|
| easy | 234 | 235 |
| hard | 229 | 229 |
| medium | 579 | 577 |

## Depth
| bucket | SFT_HALF_A | RL_POOL_B |
|---|---:|---:|
| 1 | 52 | 50 |
| 12 | 69 | 70 |
| 15 | 34 | 33 |
| 18 | 15 | 16 |
| 3 | 560 | 562 |
| 6 | 284 | 283 |
| 9 | 28 | 27 |

## Verifier
| bucket | SFT_HALF_A | RL_POOL_B |
|---|---:|---:|
| run | 360 | 363 |
| test | 682 | 678 |

## Family Difficulty Depth
| bucket (`family|difficulty|depth|verifier`) | SFT_HALF_A | RL_POOL_B |
|---|---:|---:|
| `patch_run_pass|easy|3|run` | 56 | 58 |
| `patch_run_pass|hard|3|run` | 18 | 20 |
| `patch_run_pass|medium|3|run` | 120 | 122 |
| `patch_run_recover|easy|12|run` | 2 | 1 |
| `patch_run_recover|easy|6|run` | 11 | 11 |
| `patch_run_recover|hard|12|run` | 9 | 9 |
| `patch_run_recover|hard|15|run` | 3 | 3 |
| `patch_run_recover|hard|18|run` | 5 | 5 |
| `patch_run_recover|hard|6|run` | 44 | 43 |
| `patch_run_recover|hard|9|run` | 2 | 3 |
| `patch_run_recover|medium|12|run` | 10 | 10 |
| `patch_run_recover|medium|15|run` | 4 | 4 |
| `patch_run_recover|medium|6|run` | 52 | 52 |
| `patch_run_recover|medium|9|run` | 8 | 7 |
| `patch_test_pass|easy|3|test` | 98 | 98 |
| `patch_test_pass|hard|3|test` | 66 | 64 |
| `patch_test_pass|medium|3|test` | 202 | 200 |
| `patch_test_recover|easy|12|test` | 8 | 9 |
| `patch_test_recover|easy|6|test` | 25 | 25 |
| `patch_test_recover|easy|9|test` | 3 | 2 |
| `patch_test_recover|hard|12|test` | 12 | 12 |
| `patch_test_recover|hard|15|test` | 11 | 10 |
| `patch_test_recover|hard|18|test` | 10 | 11 |
| `patch_test_recover|hard|6|test` | 46 | 45 |
| `patch_test_recover|hard|9|test` | 3 | 4 |
| `patch_test_recover|medium|12|test` | 28 | 29 |
| `patch_test_recover|medium|15|test` | 16 | 16 |
| `patch_test_recover|medium|6|test` | 106 | 107 |
| `patch_test_recover|medium|9|test` | 12 | 11 |
| `run_only|easy|1|run` | 9 | 9 |
| `run_only|medium|1|run` | 7 | 6 |
| `test_only|easy|1|test` | 22 | 22 |
| `test_only|medium|1|test` | 14 | 13 |
