# RLVR on GLYPH_SFT (full fine-tune)

Target: 2× A100 80GB (or 2× H100 80GB / 2× H200). Trainer on GPU 1, rollout vLLM + frozen teacher vLLM on GPU 0.

## 1. Clone

```bash
git clone https://github.com/JayZenith/glyph.git
cd glyph
```

## 2. Install prime-rl + flash-attn + rust + patches

```bash
bash rl/setup/install_prime_rl.sh
```

## 3. Generate Dataset 
```bash
python3 -m rl.rust.prepare_cases \
   --root runs/rlvr1/rust_cases \
   --output runs/rlvr1/prompts.jsonl \
   --phrasings 2 \
   --gold-count 12
```

## 4. Run training 

```bash
HF_HOME=/workspace/.hf_home \
CARGO_HOME=$HOME/.cargo \
RUSTUP_HOME=$HOME/.rustup \
PATH=/workspace/prime-rl-src/.venv/bin:$HOME/.cargo/bin:$PATH \
PYTHONPATH=/workspace/glyph:/workspace/glyph/rl \
CUDA_VISIBLE_DEVICES=0,1 \
python rl/train.py \
    --model JayZenith/GLYPH_SFT \
    --teacher-model JayZenith/GLYPH_SFT \
    --data runs/rlvr1/prompts.jsonl \
    --output outputs/rlvr1_runX \
    --max-steps 200 \
    --batch-size 48 \
    --rollouts-per-example 4 \
    --seq-len 5120 \
    --max-model-len 8192 \
    --max-completion-tokens 1280 \
    --learning-rate 1e-6 \
    --weight-decay 0.01 \
    --checkpoint-interval 1000 \
    --temperature 0.8 \
    --gpu-memory-utilization 0.75 \
    --teacher-gpu-memory-utilization 0.12 \
    --max-samples 95 \
    --max-trace-chars 50000 \
    --max-tool-rounds 5 \
    --tool-timeout 30 \
    --port 8000 \
    --teacher-port 8001 \
    --teacher-tau 0.0 \
    --clean-tool-boundary-bonus 1.5 \
    --structure-valid-bonus 1.0 \
    --penalty-unbalanced-braces -0.5 \
    --penalty-unbalanced-brackets -0.5 \
    --penalty-unbalanced-special-quotes -0.5 \
    --penalty-garbage-after-final-response -2.0 \
    --penalty-final-response-unclosed -1.25 \
    --penalty-missing-response -1.5 \
    --penalty-undefined-tags -0.4 \
    --penalty-unsatisfied-todos -1.5 \
    --penalty-repetition -1.0 \
    --penalty-tool-calls-without-matching-result -1.0 \
    --no-call-penalty -1.25 \
    --any-success-bonus 0.5 \
    --missing-results-penalty -1.0 \
    --response-presence-bonus 0.2

```
