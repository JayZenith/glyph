import json, glob, os
import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from tensorboard.backend.event_processing import event_accumulator

ROOT="."
OUT=f"{ROOT}/blog/assets"
os.makedirs(OUT, exist_ok=True)
INK="#1a1a1a"; BLUE="#2a6fdb"; RED="#d6452c"; GREEN="#2e9e5b"; GREY="#9aa0a6"; AMBER="#e0a000"
plt.rcParams.update({"figure.dpi":130,"font.size":11,"axes.edgecolor":"#cccccc",
    "axes.grid":True,"grid.color":"#eeeeee","axes.axisbelow":True,"savefig.bbox":"tight"})

def save(fig,name):
    fig.savefig(f"{OUT}/{name}", facecolor="white"); plt.close(fig); print("wrote",name)

# ---------- 1. SFT_V1 training loss ----------
ev=glob.glob(f"{ROOT}/results/SFT_V1/tensorboard/**/events.out*",recursive=True)[0]
ea=event_accumulator.EventAccumulator(ev); ea.Reload()
loss=ea.Scalars("train/loss")
xs=[s.step for s in loss]; ys=[s.value for s in loss]
fig,ax=plt.subplots(figsize=(7,3.6))
ax.plot(xs,ys,color=BLUE,lw=1.6)
ax.set_title("SFT_V1 — training loss (Qwen3-4B-Base, signal_1062, 3 epochs)")
ax.set_xlabel("optimizer step"); ax.set_ylabel("cross-entropy loss")
ax.set_ylim(0,max(ys[:5])*1.05)
save(fig,"fig_sft_loss.png")

# ---------- 2. SFT_V1 vs RLVR_V1 collapse ----------
def summ(p): return json.load(open(p))["summary"]["sft"]
s1=summ(f"{ROOT}/results/SFT_V1/eval_formal_heldout_69.json")
rv=summ(f"{ROOT}/results/RLVR_V1/eval_formal_heldout_69.json")
metrics=["valid_traces\n(/69)","clean_end_rate","terminal_tool\n_success_rate"]
sft_vals=[s1["valid_traces"]/69, s1["clean_end_rate"], s1["terminal_tool_success_rate"]]
rl_vals =[rv["valid_traces"]/69, rv["clean_end_rate"], rv["terminal_tool_success_rate"]]
x=np.arange(3); w=0.38
fig,ax=plt.subplots(figsize=(7,4))
b1=ax.bar(x-w/2,sft_vals,w,label="SFT_V1",color=BLUE)
b2=ax.bar(x+w/2,rl_vals,w,label="RLVR_V1 (broken reward)",color=RED)
for b,v in list(zip(b1,sft_vals))+list(zip(b2,rl_vals)):
    ax.text(b.get_x()+b.get_width()/2,v+0.015,f"{v:.2f}",ha="center",fontsize=9)
ax.set_xticks(x); ax.set_xticklabels(metrics); ax.set_ylim(0,1.12)
ax.set_title("RLVR_V1 collapse on held-out 69 — RL regressed everything SFT learned")
ax.legend(loc="upper right",framealpha=0.9)
save(fig,"fig_collapse.png")

# ---------- 3. pass@k banding ----------
band=json.load(open(f"{ROOT}/synthetic_data/passk_train134.json"))
solves=[r["solves"] for r in band]
counts=[solves.count(i) for i in range(9)]
colors=[RED]+[AMBER]*7+[GREEN]   # 0 gap, 1-7 target, 8 solved
fig,ax=plt.subplots(figsize=(7,3.8))
ax.bar(range(9),counts,color=colors,edgecolor="white")
for i,c in enumerate(counts):
    if c: ax.text(i,c+0.6,str(c),ha="center",fontsize=9)
ax.set_xlabel("solves out of k=8  (terminal cargo success, T=0.8)")
ax.set_ylabel("# train prompts")
ax.set_title("pass@k scan of 134 train prompts (SFT_V1): the band RL can move")
n_gap=counts[0]; n_tgt=sum(counts[1:8]); n_solved=counts[8]
ax.text(0.5,0.92,f"capability-gap (0): {n_gap}   rlvr-target (1–7): {n_tgt}   solved (8): {n_solved}",
        transform=ax.transAxes,ha="left",va="top",fontsize=9.5,
        bbox=dict(boxstyle="round",fc="#f7f7f7",ec="#dddddd"))
save(fig,"fig_passk_band.png")

# ---------- 4. RLVR_PASSK_25 reward + filtered fraction per step ----------
steps=[]; rmean=[]; ffrac=[]
for d in sorted(glob.glob(f"{ROOT}/results/RLVR_PASSK_25/run_default/rollouts/step_*"),
                key=lambda p:int(p.split("_")[-1])):
    rows=[json.loads(l) for l in open(f"{d}/train_rollouts.jsonl")]
    if not rows: continue
    st=int(d.split("_")[-1])
    rs=[float(r.get("reward",0) or 0) for r in rows]
    fil=[1 for r in rows if r.get("is_filtered")]
    steps.append(st); rmean.append(np.mean(rs)); ffrac.append(len(fil)/len(rows))
order=np.argsort(steps); steps=np.array(steps)[order]; rmean=np.array(rmean)[order]; ffrac=np.array(ffrac)[order]
def roll(a,w=7):
    a=np.asarray(a,float); k=np.ones(w)/w
    return np.convolve(a,k,mode="same")
fig,ax=plt.subplots(figsize=(7,3.8))
ax.plot(steps,rmean,color=BLUE,lw=1.0,alpha=0.4)
ax.plot(steps,roll(rmean),color=BLUE,lw=2.2,label="reward (7-step avg)")
ax.set_xlabel("orchestrator step"); ax.set_ylabel("mean reward / rollout",color=BLUE)
ax.tick_params(axis="y",labelcolor=BLUE)
ax2=ax.twinx(); ax2.plot(steps,roll(ffrac),color=GREY,lw=1.6,ls="--",label="filtered (7-step avg)")
ax2.set_ylabel("fraction zero-advantage",color=GREY); ax2.set_ylim(0,1); ax2.grid(False)
ax2.tick_params(axis="y",labelcolor=GREY)
ax.set_title("RLVR_PASSK_25 training — reward noisy & trendless (~36% groups: no gradient)")
save(fig,"fig_passk25_train.png")

# ---------- 5. RLVR_PASSK_25 eval: before/after + delta histogram ----------
# matched vLLM-vs-vLLM on the 39 rlvr-target prompts; derived from the committed
# per-prompt eval (results/RLVR_PASSK_25/passk_eval_39.json), not hardcoded.
from collections import Counter
ev=json.load(open(f"{ROOT}/results/RLVR_PASSK_25/passk_eval_39.json"))
sft_mean=ev["sft_v1_mean_pass_at_k"]; s25_mean=ev["rlvr_passk_25_mean_pass_at_k"]
delta=dict(Counter(p["rlvr_passk_25_solves"]-p["sft_v1_solves"] for p in ev["prompts"]))
net=round((s25_mean-sft_mean)*100,1)
fig,(axL,axR)=plt.subplots(1,2,figsize=(10,3.8),gridspec_kw={"width_ratios":[1,1.5]})
axL.bar(["SFT_V1","RLVR_PASSK_25\n(step 25)"],[sft_mean,s25_mean],color=[BLUE,RED],width=0.55)
for i,v in enumerate([sft_mean,s25_mean]): axL.text(i,v+0.006,f"{v:.3f}",ha="center",fontsize=10)
axL.set_ylim(0.8,0.95); axL.set_ylabel("mean pass@k on 39 targets")
axL.set_title(f"capability lift attempt: net {net:+.1f} pts")
ks=sorted(delta); vals=[delta[k] for k in ks]
bcol=[RED if k<0 else (GREY if k==0 else GREEN) for k in ks]
axR.bar(ks,vals,color=bcol,edgecolor="white",width=0.8)
for k,v in zip(ks,vals): axR.text(k,v+0.4,str(v),ha="center",fontsize=9)
axR.set_xlabel("Δ solves per prompt (step25 − SFT_V1, k=8)")
axR.set_ylabel("# prompts"); axR.set_xticks(ks)
axR.set_title("6 up · 13 down · 20 flat — churn, not lift")
save(fig,"fig_passk25_eval.png")

print("\nSFT_V1:",{k:round(s1[k],3) for k in ["valid_traces","clean_end_rate","terminal_tool_success_rate"]})
print("RLVR_V1:",{k:round(rv[k],3) for k in ["valid_traces","clean_end_rate","terminal_tool_success_rate"]})
print("band counts 0..8:",counts)
print("passk25 steps:",len(steps),"final reward:",round(float(rmean[-1]),2),"mean filtered:",round(float(ffrac.mean()),2))
