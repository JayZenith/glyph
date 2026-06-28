# Glyph: a verifiable RL environment for a Rust tool-use agent

**TL;DR.** I built a verifiable-reward RL environment + eval suite for a Rust
tool-use coding agent (Qwen3-4B), ran SFT then RLVR, found RLVR flat, **diagnosed
why** (the sparse reward gave no gradient on the hard tail), fixed it with a
**dense partial-credit reward**, and measured a small but **reproducible** pass@8
lift — with seed replication, not a single lucky run.

Code: <https://github.com/JayZenith/glyph> · Models: `JayZenith/SFT_HALF_A_V8`,
`JayZenith/RLVR_VFINAL_STEP{10,20,30}`

## What Glyph is

A `verifiers` / PRIME-RL environment. Each task hands the model a real Rust crate
and a tool-use job: patch until `cargo_test` passes, patch until `cargo_run`
prints exact stdout, or run an already-correct crate. The model emits
`CALL tool {...}`, tools execute against real cargo, and it must end with a clean
`FINAL`. The reward is verifiable — cargo actually compiles and runs.
`rl/task_trace.py` exposes `load_environment() -> vf.Environment`, the
Environments-Hub-standard shape.

Success is strict `valid_trace`: terminal cargo success **+** a single clean
`FINAL` after it **+** exact `CALL` syntax **+** no tool use after success. Cargo
passing mid-trace is not success if the trace is unusable.

Held-out eval: 150 unseen crates, disjoint from SFT/RL data (split by `case_id`,
0 leakage). Mix: 53 `patch_test_recover`, 42 `patch_run_pass`, 34
`patch_test_pass`, 11 `patch_run_recover`, 5 `run_only`, 5 `test_only`.

## SFT built the agent

SFT (`Qwen3-4B-Base` → `SFT_HALF_A_V8`) learned the protocol and a useful repair
prior. Greedy strict pass@1: **74/150**.

## RLVR with a sparse reward was flat

LoRA RLVR via PRIME-RL (on-policy distillation anchor to the SFT teacher +
verifiable reward). Reward: `+10` only for a clean verifier success, penalties
otherwise. Strict pass@1, same harness: **74 → 72 / 150**. Flat-to-down.

## Why it was flat (the useful part)

I analyzed the failures instead of just the score:

- **Structure is 100% SFT-saturated.** Across all 150: exact `CALL` syntax, call
  IDs, and cargo paths are all 150/150. RLVR has zero headroom on format.
- **Failures are capability, not hygiene.** 75 of 76 failures never reach a
  terminal cargo success — the model formats perfectly and writes the wrong Rust.
- **72/150 fail in both SFT and RLVR** — a shared hard tail; RLVR only reshuffled
  ~10 borderline prompts.
- **The hard tail produces no gradient.** RLVR learns from reward *variance*
  within each group of 8 rollouts. When all 8 fail, a binary reward scores them
  identically → zero advantage → the group is filtered → no gradient. The prompts
  that should teach the most contribute nothing. (At step 0 of the sparse run the
  orchestrator filtered the entire first batch.)
- **But ~half the hard tail is partially correct:** of the 75 cargo-failures,
  52% compile and 44% pass ≥1 test (median 50%). Under a binary reward these
  score identically to never-compiles.

## The fix: a dense partial-credit reward

Give graded credit in the no-success region only: a small bonus for compiling,
plus a bonus scaled by the test-pass fraction. Both are fixed by the task (so
unhackable) and capped well below the `+10` success bonus. Now 8 failing rollouts
get *different* scores → non-zero advantage → gradient on exactly the prompts
that were dead weight before.

`rl/reward.py`, off by default, enabled with
`--progress-compile-bonus 0.5 --progress-test-frac-bonus 2.0`.

## Measuring honestly

Greedy pass@1 is too noisy to detect a small effect, so I used pass@8 (T=0.8)
with **3-seed replication** per model.

| model | valid@8 per seed | mean |
| --- | --- | ---: |
| `SFT_HALF_A_V8` | 95, 97, 100 | 97.3 |
| + dense RLVR (step 10) | 102, 102, 99 | **101.0** |

**Δ ≈ +3.7 valid@8, seed-level t-test p ≈ 0.06.** Small and borderline, but
reproducible: dense-RLVR never drops below 99, SFT never exceeds 100. Stability
(8/8) is flat.

A single run had shown +7. Replication revealed SFT alone swings 95–100, so the
+7 was partly seed luck — the honest effect is ~+4, not +7. The sparse run was
flat, so the lift is attributable to the reward change, not to RLVR in general.

## What I claim / don't

- **Claim:** a dense, unhackable partial-credit reward turns a flat RLVR result
  into a small, reproducible pass@8 lift, by restoring gradient on the hard tail.
- **Don't claim:** a large RLVR win, or significance at p<0.05. It's a small
  effect on a 4B LoRA, measured honestly.

## Reusable lessons

- Verifier RL only works if the verifier matches the full behavior you want — for
  agents, the contract is the whole trace, not just "cargo passed."
- A binary verifiable reward silently discards the hard tail via zero-advantage
  filtering. Dense, unhackable partial credit recovers it.
- Greedy pass@1 hides small effects in noise; pass@k + seed replication is the
  minimum honest bar (a single-run +7 here was seed luck).
- Export the *served* policy: `run_default/broadcasts/step_N` as a PEFT adapter,
  not `weights/step_N`.

Reproduction commands are in the [README](../README.md).
