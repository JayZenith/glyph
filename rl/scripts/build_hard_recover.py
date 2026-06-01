#!/usr/bin/env python3
"""Build rl_prompts_hard_recover.jsonl from rl_prompts_1062.jsonl.

Goals: drop easy all-FINAL prompts, concentrate on borderline recover tasks,
oversample near-twins of the failing canaries, and prefer prompts that showed
mixed (high-variance) rollout outcomes in the v6 run.
"""
import json, os, re
from collections import defaultdict

SRC = "synthetic_data/rl_prompts_1062.jsonl"
OUT = "synthetic_data/rl_prompts_hard_recover.jsonl"
V6  = "outputs/rlvr_final_penaltyv6/run_default/rollouts"

RECOVER = {"patch_test_recover", "patch_run_recover"}
PASS    = {"patch_test_pass", "patch_run_pass"}

# Concept token sets for the FOUR failing canaries (step-75 failures).
CANARY_FAIL = [
    {"route", "action", "enum", "branch"},
    {"layered", "flags", "config", "cli", "precedence"},
    {"department", "report", "filters", "formats", "monthly"},
    {"csv", "record", "required", "key", "validate"},
]
STOP = {"patch", "run", "test", "pass", "recover", "only", "case", "fix", "and",
        "the", "of", "to", "a", "b", "c", "with", "for"}

def toks(case_id):
    return set(re.findall(r"[a-z]+", case_id)) - STOP

def is_canary_twin(case_id):
    t = toks(case_id)
    return any(len(t & c) >= 3 for c in CANARY_FAIL)

# ---- join v6 outcomes by case_id: mixed vs trivially-all-FINAL ----
final_by_case = defaultdict(list)
if os.path.isdir(V6):
    for step in sorted(os.listdir(V6)):
        fp = os.path.join(V6, step, "train_rollouts.jsonl")
        if not os.path.isfile(fp):
            continue
        for l in open(fp):
            d = json.loads(l)
            cid = os.path.basename((d.get("info") or {}).get("blueprint_root", "")) or str(d.get("example_id"))
            full = d["prompt"][0]["content"] + "".join((m.get("content") or "") for m in d["completion"])
            final_by_case[cid].append("FINAL:" in full)
mixed_ids   = {c for c, v in final_by_case.items() if len(v) > 1 and any(v) and not all(v)}
allfinal_ids = {c for c, v in final_by_case.items() if len(v) > 1 and all(v)}

# ---- assign a copy-count weight per source row ----
rows = [json.loads(l) for l in open(SRC)]
out = []
stats = defaultdict(int)
for idx, r in enumerate(rows):
    kind = r.get("kind", "")
    diff = (r.get("difficulty") or "medium").lower()
    cid  = r.get("case_id", "")
    twin = is_canary_twin(cid)
    w = 0
    if kind in RECOVER:
        w = 1
        if diff == "hard":        w += 1   # more borderline recover
        if twin:                  w += 2   # oversample failing-canary twins
        if cid in mixed_ids:      w += 1   # prefer high-variance prompts
        if cid in allfinal_ids:   w = max(1, w - 1)  # solved-trivially: fewer
    elif kind in PASS:
        # downsample easy pass-kind
        if diff == "easy":        w = 0
        elif diff == "medium":    w = 1 if idx % 3 == 0 else 0
        else:                     w = 1   # keep hard pass
        if cid in allfinal_ids:   w = 0   # drop observed-trivial pass
        if twin:                  w = max(w, 1) + 1  # keep+oversample failing twins
        if cid in mixed_ids:      w += 1
    else:
        w = 0  # drop test_only / other non-recover easy filler

    if w > 0:
        out.extend([r] * w)
    stats[f"src_{kind or 'other'}"] += 1
    if twin and w: stats["twin_rows_kept"] += 1
    if w: stats["unique_kept"] += 1

with open(OUT, "w") as f:
    for r in out:
        f.write(json.dumps(r) + "\n")

# ---- summary ----
out_kind = defaultdict(int)
for r in out: out_kind[r.get("kind", "other")] += 1
rec = sum(v for k, v in out_kind.items() if k in RECOVER)
print(f"wrote {OUT}: {len(out)} rows from {len(rows)} source rows")
print(f"unique source prompts kept: {stats['unique_kept']}")
print(f"v6 join: mixed-outcome cases={len(mixed_ids)}, all-FINAL(trivial) cases={len(allfinal_ids)}")
twin_cases = sorted({r.get("case_id","") for r in rows if is_canary_twin(r.get("case_id",""))})
print(f"failing-canary twin source rows kept: {stats['twin_rows_kept']}")
print(f"distinct twin cases ({len(twin_cases)}):")
for c in twin_cases: print(f"    {c}")
print("output kind composition (with copies):")
for k in sorted(out_kind):
    print(f"  {k:22s} {out_kind[k]:>5}  ({out_kind[k]/len(out)*100:.0f}%)")
print(f"recover share: {rec}/{len(out)} = {rec/len(out)*100:.0f}%")
