#!/usr/bin/env bash
# Full-finetune RLVR launcher. Targets 2-GPU 80-96GB boxes.
# Trainer on GPU 1, rollout vLLM (+ optional teacher anchor) on GPU 0.
# HW_PROFILE selects seq_len / completion / tool_rounds presets per arch.
# Auto-detect by default; override with HW_PROFILE=a100-80gb or blackwell-96gb.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PRIME_RL_DIR="${PRIME_RL_DIR:-/workspace/prime-rl-src}"
MODEL="${MODEL:-JayZenith/GLYPH-SFT-V2}"
TEACHER_MODEL="${TEACHER_MODEL:-$MODEL}"
TEACHER_TAU="${TEACHER_TAU:-0.0}"
TEACHER_ANCHOR="${TEACHER_ANCHOR:-0}"
DATA_PATH="${DATA_PATH:-$ROOT_DIR/runs/rlvr1/prompts.jsonl}"
OUTPUT_DIR="${OUTPUT_DIR:-$ROOT_DIR/outputs/rlvr1}"
PORT="${PORT:-8000}"
TEACHER_PORT="${TEACHER_PORT:-8001}"
# Hardware profile — picks seq_len / completion / tool_rounds tuned for the
# trainer GPU's VRAM. HW_PROFILE=auto detects from nvidia-smi GPU name.
# Any individual knob below can still be overridden via its own env var.
HW_PROFILE="${HW_PROFILE:-auto}"
if [[ "$HW_PROFILE" == "auto" ]]; then
  GPU_NAME="$(nvidia-smi --query-gpu=name --format=csv,noheader 2>/dev/null | head -1)"
  case "$GPU_NAME" in
    *"A100"*)                                 HW_PROFILE=a100-80gb ;;
    *"H100"*)                                 HW_PROFILE=a100-80gb ;;
    *"RTX PRO 6000"*|*"H200"*|*"B200"*)       HW_PROFILE=blackwell-96gb ;;
    *)                                        HW_PROFILE=a100-80gb ;;  # conservative
  esac
fi

case "$HW_PROFILE" in
  a100-80gb)
    DEFAULT_SEQ_LEN=5120
    DEFAULT_MAX_COMPLETION_TOKENS=1024
    DEFAULT_MAX_TOOL_ROUNDS=4
    ;;
  blackwell-96gb)
    DEFAULT_SEQ_LEN=6144
    DEFAULT_MAX_COMPLETION_TOKENS=1024
    DEFAULT_MAX_TOOL_ROUNDS=5
    ;;
  *)
    echo "Unknown HW_PROFILE=$HW_PROFILE. Valid: a100-80gb, blackwell-96gb." >&2
    exit 1
    ;;
esac

SEQ_LEN="${SEQ_LEN:-$DEFAULT_SEQ_LEN}"
MAX_MODEL_LEN="${MAX_MODEL_LEN:-$SEQ_LEN}"
MAX_COMPLETION_TOKENS="${MAX_COMPLETION_TOKENS:-$DEFAULT_MAX_COMPLETION_TOKENS}"
MAX_TOOL_ROUNDS="${MAX_TOOL_ROUNDS:-$DEFAULT_MAX_TOOL_ROUNDS}"

echo "[run] HW_PROFILE=$HW_PROFILE  SEQ_LEN=$SEQ_LEN  MAX_COMPLETION_TOKENS=$MAX_COMPLETION_TOKENS  MAX_TOOL_ROUNDS=$MAX_TOOL_ROUNDS"

source "$PRIME_RL_DIR/.venv/bin/activate"

export PYTHONPATH="$ROOT_DIR:$ROOT_DIR/rl${PYTHONPATH:+:$PYTHONPATH}"
export CUDA_VISIBLE_DEVICES="${CUDA_VISIBLE_DEVICES:-0,1}"
export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
export RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}"
export PATH="$CARGO_HOME/bin:$PATH"

ARGS=(
  python3 "$ROOT_DIR/rl/train.py"
  --model "$MODEL"
  --port "$PORT"
  --data "$DATA_PATH"
  --output "$OUTPUT_DIR"
  --gpu-memory-utilization 0.7
  --seq-len "$SEQ_LEN"
  --max-model-len "$MAX_MODEL_LEN"
  --max-completion-tokens "$MAX_COMPLETION_TOKENS"
  --max-tool-rounds "$MAX_TOOL_ROUNDS"
)

if [[ "$TEACHER_ANCHOR" == "1" ]]; then
  ARGS+=(--teacher-anchor --teacher-model "$TEACHER_MODEL" --teacher-tau "$TEACHER_TAU" --teacher-port "$TEACHER_PORT")
fi

exec "${ARGS[@]}"
