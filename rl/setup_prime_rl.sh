#!/usr/bin/env bash
set -euo pipefail

# Install PRIME-RL the way upstream expects: its own uv-managed Python 3.12 env.
# Then patch the installed checkout so it can initialize trainer LoRA + lm_head
# from a PEFT adapter such as JayZenith/glyph-sft-v1-adapter.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PRIME_RL_DIR="${1:-$HOME/prime-rl}"
PRIME_PYTHON_VERSION="${PRIME_PYTHON_VERSION:-3.12}"

install_flash_attn_wheel() {
  local python_bin="$1"
  local torch_version cuda_version py_tag abi_flag wheel_url

  torch_version="$("$python_bin" - <<'PY'
import torch
print(torch.__version__.split("+", 1)[0])
PY
)"
  cuda_version="$("$python_bin" - <<'PY'
import torch
print((torch.version.cuda or "").replace(".", ""))
PY
)"
  py_tag="$("$python_bin" - <<'PY'
import sys
print(f"cp{sys.version_info.major}{sys.version_info.minor}")
PY
)"
  abi_flag="$("$python_bin" - <<'PY'
import torch
print("TRUE" if torch._C._GLIBCXX_USE_CXX11_ABI else "FALSE")
PY
)"

  if [ -n "${FLASH_ATTN_WHEEL_URL:-}" ]; then
    wheel_url="$FLASH_ATTN_WHEEL_URL"
  elif [ "$torch_version" = "2.11.0" ] && [ "$cuda_version" = "128" ] && [ "$py_tag" = "cp312" ] && [ "$abi_flag" = "TRUE" ]; then
    wheel_url="https://github.com/lesj0610/flash-attention/releases/download/v2.8.3-cu12-torch2.11/flash_attn-2.8.3%2Bcu12torch2.11cxx11abiTRUE-cp312-cp312-linux_x86_64.whl"
  else
    echo "No pinned flash-attn wheel for torch=$torch_version cuda=$cuda_version python=$py_tag abi=$abi_flag" >&2
    echo "Set FLASH_ATTN_WHEEL_URL to a matching wheel before rerunning rl/setup_prime_rl.sh." >&2
    return 1
  fi

  uv pip install --python "$python_bin" peft
  uv pip install --python "$python_bin" "$wheel_url"
}

if ! command -v uv >/dev/null 2>&1; then
  python3 -m pip install uv
fi

export PATH="$HOME/.local/bin:$PATH"

uv python install "$PRIME_PYTHON_VERSION"

if [ ! -d "$PRIME_RL_DIR/.git" ]; then
  git clone https://github.com/PrimeIntellect-ai/prime-rl.git "$PRIME_RL_DIR"
fi

cd "$PRIME_RL_DIR"
git pull --ff-only
uv sync --python "$PRIME_PYTHON_VERSION"

install_flash_attn_wheel "$PRIME_RL_DIR/.venv/bin/python"

SITE_PACKAGES_DIR="$("$PRIME_RL_DIR/.venv/bin/python" -c 'import pathlib, prime_rl; print(pathlib.Path(prime_rl.__file__).resolve().parent)')"
"$PRIME_RL_DIR/.venv/bin/python" "$ROOT_DIR/rl/patch_install.py" "$SITE_PACKAGES_DIR"

cat <<EOF
PRIME-RL ready at: $PRIME_RL_DIR
Activate with:
  source "$PRIME_RL_DIR/.venv/bin/activate"

Run glyph RL wrappers from:
  cd "$ROOT_DIR"
EOF
