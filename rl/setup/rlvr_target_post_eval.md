# RLVR target — post-eval

How we score the capability-lift run. Pairs with `rl_setup_training.md`.

## Reward (what RL is optimizing)

`rl/task_trace.py`, one scalar per rollout — verifier-dominant, termination tails
zeroed:

```text
verifier passed              +8.0   <- the whole signal
structure valid              +0.5   } format floor
no tool call                 -2.0   }
malformed call (per, cap 4)  -1.0   }
FINAL / churn / round-cap     0.0   <- zeroed (not this run's goal)
```

Best ≈ +8.5 (solve), unsolved ≈ +0.5 (format floor only). GRPO gets its gradient
from within-group variance: on a partial-solve prompt some of the 8 rollouts hit
+8.5 and some +0.5, and the advantage pushes the policy toward the solving ones.
That's why we train ONLY the partial-solve band — a prompt that's 8/8 or 0/8 has
no variance, zero advantage, no signal.

## The goal

Lift each of the 39 rlvr-target cases toward **8/8** (pass@k = 1.0). They're
partial today, so every one has headroom and a gradient.

Baseline (SFT_V1, k=8, T=0.8, in `synthetic_data/passk_train134.json`):

```text
targets            39
mean pass@8        0.817   (255/312 rollouts solve)
distribution       1×4/8 · 4×5/8 · 7×6/8 · 27×7/8
```

Most are 7/8 (one rollout short), so the realistic win is mean pass@8 0.817 → ~0.95+
and the count of cases reaching 8/8 climbing. Modest by design — this slice has no
0/8 walls, it's near-solved cases nudged over the line.

## Post-eval = re-scan the same 39

Identical scan, swap the model. Run per checkpoint on a separate 1-GPU box:

```bash
env HF_HOME=/workspace/.hf_home CUDA_VISIBLE_DEVICES=0 PYTHONPATH=/workspace/glyph \
  /workspace/prime-rl-src/.venv/bin/python sft/passk_scan.py \
    --sft-model outputs/rlvr_passk/weights/step_25 \
    --prompt-file runs/rlvr_passk_train150/prompts.yaml \
    --prompt-section train_passk_scan_134 \
    --cases-root runs/rlvr_passk_train150/cases \
    -k 8 --temperature 0.8 \
    --output outputs/rlvr_passk/passk_step25.json
```

Compare on the **identical 39 target names** (the scan covers all 134; filter to
the targets so before/after use the same set).

## Report

Per checkpoint vs SFT_V1 baseline:

```text
metric                 SFT_V1   step_25   ...
mean pass@8 (39)        0.817      ?
cases now 8/8           0/39       ?
cases regressed (<base) —          ?       <- must stay ~0
```

- **Win:** mean pass@8 up, more cases at 8/8, ~0 regressions.
- **Kill:** mean pass@8 below baseline for 2 consecutive checkpoints (collapse
  signature). Best checkpoint is usually early (~step 25).
- **Artifact:** the before→after delta on the 39, labeled **in-set** (no held-out
  split for v1).

## Honest caveat

Same k and T as the baseline, so the comparison is fair. But these are in-set
(trained on) and near-solved already — the claim is "RLVR lifts solve-rate on the
partial-solve band," not generalization. A future version adds a stratified
held-out split for a generalization claim.
