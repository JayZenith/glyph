# Glyph

Glyph is a Rust tool-use agent experiment.

The model emits `CALL tool(...)` blocks, tools execute against real Rust crates,
and the model is expected to stop with a clean `FINAL`. The main lesson from
this repo is that verifier RL is only meaningful when the reward, tool protocol,
model export, and held-out eval all enforce the same trace contract.

## Final Status

The SFT model is the strongest aggregate result.

| Model / checkpoint | Held-out-69 strict `valid_trace` | Notes |
| --- | ---: | --- |
| `SFT_V1` | 52/69 | Best broad SFT result |
| `SFT_V2` | 48/69 | More recovery data, worse broad score |
| `SFT_V3` | 50/69 | More hard-tail capability, no broad improvement |
| `SFT_HALF_A` | 51/69 | Clean split baseline |
| `RLVR_V1000` step 25 direct merge | 50/69 | One SFT miss flipped to pass, two SFT solves regressed |
| `RLVR_V3000` step 5 adapter | 44/69 | Clean adapter eval, regressed |
| `RLVR_V3000` step 10 adapter | 47/69 | Best V3000 checkpoint checked locally |
| `RLVR_V3000` step 15 adapter | 43/69 | Drifted down |

The honest RLVR finding:

```text
RLVR showed one strict held-out recovery signal absent from SFT_HALF_A greedy
behavior, but aggregate reliability regressed.
```

The case-level signal was:

```text
eval100_039_select_event_codes_partial_then_full_fix
```

`SFT_HALF_A` looped through repeated repair attempts and exhausted the budget.
`RLVR_V1000` step 25 made a better fourth patch, passed `cargo_test`, and
emitted a clean final. Two other recovery cases regressed, so this is a signal
that needs verification, not a win.

## Strict Success Metric

The metric that matters is strict held-out `valid_trace`.

```text
valid_trace =
  terminal cargo_test or cargo_run success
  + clean FINAL after that verifier success
  + exact CALL syntax
  + no role-marker leakage
  + no repetition or gibberish
  + no extra tool use after successful verification
```

Do not use loose terminal-tool success as the main claim. It can count
successful non-verifier tools and overstate progress.

## Data

The clean experiment split `synthetic_data/signal_v3.jsonl` into two
deterministic, non-overlapping halves.

| File | Purpose | Size |
| --- | --- | ---: |
| `synthetic_data/signal_v3_sft_half_a.jsonl` | SFT traces | 1,042 rows, 762 unique cases |
| `synthetic_data/signal_v3_rl_pool_b.jsonl` | held-out RL trace pool | 1,041 rows, 760 unique cases |
| `synthetic_data/rl_prompts_signal_v3_pool_b.jsonl` | RL prompt manifest for `rl/train.py` | 760 prompts |
| `synthetic_data/signal_v3_rl_pool_b_prompts.yaml` | pass@k-compatible prompt manifest | 760 prompts |
| `synthetic_data/signal_v3_split_summary.json` | split audit | summary |
| `synthetic_data/signal_v3_split_summary.md` | split audit | summary |

Leakage checks:

```text
case_id_overlap = 0
trace_overlap = 0
```

Important: `rl/train.py` takes the RL prompt manifest, not the SFT trace JSONL.

```text
Use:       synthetic_data/rl_prompts_signal_v3_pool_b.jsonl
Do not use synthetic_data/signal_v3_rl_pool_b.jsonl directly for RL.
```

## SFT Setup

Tested SFT/eval setup is the repo venv created by:

```bash
git clone https://github.com/JayZenith/glyph.git
cd glyph
git pull --ff-only

bash sft/setup/install_sft_env.sh
source .venv/bin/activate
```

The installer pins a CUDA Torch/vLLM-compatible environment for SFT and formal
eval.

## Train `SFT_HALF_A`

```bash
python -m sft.train \
  --model Qwen/Qwen3-4B-Base \
  --tokenizer Qwen/Qwen3-4B-Base \
  --data synthetic_data/signal_v3_sft_half_a.jsonl \
  --output runs/SIGNAL_v3_HALF_A_SFT_E3_LR2E5 \
  --epochs 3 \
  --batch-size 1 \
  --grad-accum 8 \
  --lr 2e-5 \
  --max-seq-length 12000 \
  --no-train-split \
  --gradient-checkpointing
```

Observed result:

```text
SFT_HALF_A strict held-out-69 valid_trace = 51/69
```

## Eval `SFT_HALF_A`

This is the baseline held-out command. The eval uses greedy decoding and real
tool execution.

```bash
mkdir -p results/SFT_HALF_A

python -m sft.eval_formal \
  --sft-model runs/SIGNAL_v3_HALF_A_SFT_E3_LR2E5/final \
  --train-data synthetic_data/signal_v3_sft_half_a.jsonl \
  --prompt-file sft/evals/eval_prompts_heldout_69.yaml \
  --prompt-section post_eval_heldout_69 \
  --cases-root runs/heldout69_sft_half_a \
  --output results/SFT_HALF_A/eval_formal_heldout_69.json \
  --max-new-tokens 4000 \
  --max-tool-rounds 20 \
  --prompt-batch-size 4 \
  --tool-workers 8
```

`--max-new-tokens 4000` is the per-assistant-turn generation cap. The trace can
span multiple assistant turns up to `--max-tool-rounds`.

If VRAM is tight, reduce `--prompt-batch-size` to `2` or omit batching.

## Optional Pass@k Diagnostics

RL pool pass@4:

```bash
python -m sft.passk_scan_vllm \
  --sft-model runs/SIGNAL_v3_HALF_A_SFT_E3_LR2E5/final \
  --prompt-file synthetic_data/signal_v3_rl_pool_b_prompts.yaml \
  --prompt-section rl_pool_b \
  --cases-root runs/passk_signal_v3_rl_pool_b_sft_half_a \
  -k 4 \
  --temperature 0.8 \
  --max-new-tokens 4000 \
  --max-tool-rounds 15 \
  --output results/SFT_HALF_A/passk_rl_pool_b_k4.json \
  --save-rollouts
```

Held-out-69 pass@4:

```bash
python -m sft.passk_scan_vllm \
  --sft-model runs/SIGNAL_v3_HALF_A_SFT_E3_LR2E5/final \
  --prompt-file sft/evals/eval_prompts_heldout_69.yaml \
  --prompt-section post_eval_heldout_69 \
  --cases-root runs/passk_heldout69_sft_half_a \
  -k 4 \
  --temperature 0.8 \
  --max-new-tokens 4000 \
  --max-tool-rounds 20 \
  --output results/SFT_HALF_A/passk_heldout69_k4.json \
  --save-rollouts
```

## RLVR Setup

The RLVR runs used a 4-GPU node:

```text
GPU 0,1,2: PRIME-RL worker pool
GPU 3:     teacher model
```

The command topology was:

```text
--prime-rl-gpu-ids 0,1,2
--num-infer-gpus 1
--num-train-gpus 2
--teacher-device 3
--gpus-per-node 3
```

Install:

```bash
git clone https://github.com/JayZenith/glyph.git
cd glyph
git pull --ff-only

PRIME_RL_ENABLE_LORA=1 bash rl/setup/install_prime_rl.sh
source /workspace/prime-rl-src/.venv/bin/activate
```

Sanity requirements before a real run:

```text
1. W&B samples show literal <|im_start|>assistant / CALL / tool / RESULT format.
2. Logs show zero_advantage, gibberish, and repetition filters enabled.
3. The reward gives top score only to held-out-style valid traces.
4. Checkpoints are exported from run_default/broadcasts/step_N, not weights/step_N.
```

## Final RLVR Command Used For V3000

```bash
python rl/train.py \
  --model JayZenith/SFT_HALF_A \
  --teacher-model JayZenith/SFT_HALF_A \
  --lora-rank 64 \
  --lora-alpha 128 \
  --lora-dropout 0.0 \
  --lora-name glyph-signal-v3000-pool-b-r64-a128 \
  --data synthetic_data/rl_prompts_signal_v3_pool_b.jsonl \
  --output outputs/RLVR_SIGNAL_V3000_POOL_B_LORA_R64_A128 \
  --max-steps 40 \
  --batch-size 48 \
  --rollouts-per-example 8 \
  --seq-len 8192 \
  --max-model-len 16384 \
  --teacher-max-model-len 16384 \
  --max-completion-tokens 4000 \
  --learning-rate 2e-7 \
  --weight-decay 0.01 \
  --checkpoint-interval 5 \
  --temperature 0.8 \
  --teacher-tau 0.8 \
  --max-tool-rounds 15 \
  --tool-timeout 30 \
  --activation-checkpointing \
  --fused-lm-head-token-chunk-size auto \
  --gpu-memory-utilization 0.70 \
  --teacher-gpu-memory-utilization 0.50 \
  --prime-rl-gpu-ids 0,1,2 \
  --num-infer-gpus 1 \
  --num-train-gpus 2 \
  --gpus-per-node 3 \
  --port 8000 \
  --teacher-port 8001 \
  --teacher-device 3 \
  --enforce-gibberish-filter \
  --enforce-repetition-filter
```

This run did not improve over SFT. The best checked V3000 checkpoint was step 10 at
`47/69`.

## Export RLVR Checkpoints

Use PEFT adapter export from the broadcast adapter.

Do not treat `weights/step_N` as the official model artifact. It is a trainer
checkpoint, not the clean served policy.

```bash
python rl/scripts/export_prime_lora_adapter.py \
  --base-model JayZenith/SFT_HALF_A \
  --adapter-dir outputs/RLVR_SIGNAL_V3000_POOL_B_LORA_R64_A128/run_default/broadcasts/step_10 \
  --output outputs/RLVR_SIGNAL_V3000_POOL_B_LORA_R64_A128/adapter_step_10
```

Upload the exported adapter directory:

```bash
huggingface-cli upload JayZenith/RLVR_V3000_STEP10 \
  outputs/RLVR_SIGNAL_V3000_POOL_B_LORA_R64_A128/adapter_step_10 .
```

The adapter export should contain:

```text
adapter_config.json
adapter_model.safetensors
prime_lora_adapter_export.json
```

## Eval RLVR Adapters

Evaluate adapters by loading the SFT base plus `--sft-adapter`.

```bash
mkdir -p results/RLVR_V3000_STEP10

python -m sft.eval_formal \
  --sft-model JayZenith/SFT_HALF_A \
  --sft-adapter JayZenith/RLVR_V3000_STEP10 \
  --train-data synthetic_data/signal_v3_sft_half_a.jsonl \
  --prompt-file sft/evals/eval_prompts_heldout_69.yaml \
  --prompt-section post_eval_heldout_69 \
  --cases-root runs/heldout69_rlvr_v3000_step10_adapter \
  --output results/RLVR_V3000_STEP10/eval_formal_heldout_69.json \
  --max-new-tokens 4000 \
  --max-tool-rounds 20 \
  --prompt-batch-size 8 \
  --tool-workers 16
```

Use the same command for step 5, 15, 20, etc. Change only:

```text
--sft-adapter
--cases-root
--output
```

## Canary Eval During RLVR

For future runs, export and canary every checkpoint interval.

```bash
python rl/scripts/export_prime_lora_adapter.py \
  --base-model JayZenith/SFT_HALF_A \
  --adapter-dir outputs/<RUN>/run_default/broadcasts/step_<N> \
  --output outputs/<RUN>/adapter_step_<N>

python -m sft.eval_formal \
  --sft-model JayZenith/SFT_HALF_A \
  --sft-adapter outputs/<RUN>/adapter_step_<N> \
  --train-data synthetic_data/signal_v3_sft_half_a.jsonl \
  --prompt-file sft/evals/eval_prompts_heldout_69.yaml \
  --prompt-section post_eval_heldout_69 \
  --cases-root runs/canary_step_<N> \
  --output results/canary_step_<N>.json \
  --max-new-tokens 4000 \
  --max-tool-rounds 20 \
  --limit 16 \
  --prompt-batch-size 4 \
  --tool-workers 8
```

Gate on:

```text
exact CALL syntax remains clean
valid_trace does not fall below best-so-far by more than two cases twice in a row
```

## Known Traps

Do not use `weights/step_N` for the official checkpoint. Use
`run_default/broadcasts/step_N` and export a PEFT adapter.

Do not claim success from `terminal_tool_success`. Use strict `valid_trace`.

Do not change the chat/tool protocol between SFT, RL, and eval. The model is
trained on the literal format.

Do not evaluate a full merged model unless you also smoke-test exact call syntax
and diff it against a known-good adapter export. The safest path is base model
plus PEFT adapter.

Do not confuse eval defaults with the official held-out command. The reported
held-out results here used `--max-tool-rounds 20` and `--max-new-tokens 4000`.

## Next Experiment, If This Continues

Do not run more blind RL on the full pool.

The next credible attempt should:

```text
1. Run pass@k over RL_POOL_B using SFT_HALF_A.
2. Keep examples with mixed outcomes under the same eval budget.
3. Drop always-solved and always-failed examples.
4. Train with frequent adapter canaries.
5. Stop early on held-out degradation.
```

The likely missing ingredient is better RL signal selection, not simply more
steps.

## Artifact Pointers

Important local artifacts:

```text
new_results/SFT_HALF_A/eval_formal_heldout_69.json
results/RLVR_V1000/eval/eval_formal_heldout_69_direct_merged.json
new_results/RLVR_V3000_STEP10/
new_results/RLVR_V3000_STEP15/
new_results/RLVR_V3000_STEP20/
blog/finalized_blog.md
```

Important Hugging Face repos:

```text
JayZenith/SFT_HALF_A
JayZenith/SFT_HALF_A_DATASET
JayZenith/RLVR_V3000_STEP5_ADAPTER
JayZenith/RLVR_V3000_STEP10
JayZenith/RLVR_V3000_STEP15
JayZenith/RLVR_V3000_STEP20
```

## Final Takeaway

SFT made a real Rust tool-use agent. RLVR showed it could change a hard
recovery trajectory into a strict held-out success, but it did not improve the
overall model. The main engineering value was forcing the reward, protocol,
export path, and eval to become exact.
