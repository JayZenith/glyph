"""
Render structured message objects into Glyph ChatML text.

This file is not a raw model-output repair layer. If the model omits a required
ChatML boundary during generation, scoring/validation should treat that as a
model failure. These helpers are only for turning stored structured messages
from JSONL/YAML/runtime state into the exact text format the model should see.

Direct users:
- sft/evals/prompt_loader.py: renders eval YAML prompts.
- sft/evals/generation.py: injects rendered tool RESULT turns during eval.
- sft/passk_scan_vllm.py: injects rendered tool RESULT turns during pass@k.
- rl/task_trace.py: renders RL transcripts/tool RESULT turns for scoring.
- rl/train.py: installs the same ChatML template into the tokenizer view.
- reward_golden_tests.py: locks the renderer/template behavior in tests.
"""

from __future__ import annotations

from typing import Any


DEFAULT_SYSTEM_PROMPT = "You are a Rust coding agent. Use tools when needed. After FINAL, stop immediately."

# This is the tokenizer-side template. rl/train.py installs it into the local
# tokenizer view so PRIME-RL/vLLM render messages the same way our Python code
# does below.
GLYPH_CHAT_TEMPLATE = """{%- for message in messages %}
{%- set role = message['role'] %}
{%- set content = message['content'] %}
{%- if role == 'assistant' %}
{{- '<|im_start|>assistant\n' + content.rstrip() }}
{%- if not content.rstrip().endswith('<|im_end|>') %}
{{- '\n<|im_end|>' }}
{%- endif %}
{{- '\n\n' }}
{%- else %}
{{- '<|im_start|>' + role + '\n' + content.rstrip() + '\n<|im_end|>\n\n' }}
{%- endif %}
{%- endfor %}
{%- if add_generation_prompt %}
{{- '<|im_start|>assistant\n' }}
{%- endif %}"""


def _message_value(message: Any, key: str, default: str = "") -> Any:
    # Most callers pass dicts from JSONL/YAML/runtime state.
    if isinstance(message, dict):
        return message.get(key, default)
    # Some library objects expose role/content as attributes.
    value = getattr(message, key, default)
    # Pydantic-style objects can expose the same fields via model_dump().
    if value is default and hasattr(message, "model_dump"):
        value = message.model_dump().get(key, default)
    return default if value is None else value


def message_role(message: Any) -> str:
    return str(_message_value(message, "role", ""))


def message_content(message: Any) -> str:
    return str(_message_value(message, "content", ""))


def render_message(role: str, content: str) -> str:
    # Convert one structured message into:
    # <|im_start|>role
    # content
    # <|im_end|>
    body = content.rstrip()
    rendered = f"<|im_start|>{role}\n{body}"
    # Stored assistant traces sometimes already include the terminal delimiter.
    # Do not double-append it. This is not repairing live model output.
    if role != "assistant" or not body.endswith("<|im_end|>"):
        rendered += "\n<|im_end|>"
    return rendered


def render_messages(messages: list[Any], add_generation_prompt: bool = False) -> str:
    # Render a whole structured transcript, with a blank line between turns.
    rendered = "".join(
        f"{render_message(message_role(message), message_content(message))}\n\n"
        for message in messages
        if message_role(message)
    )
    # Open the next assistant turn. The model generates after this marker.
    if add_generation_prompt:
        rendered += "<|im_start|>assistant\n"
    return rendered


def render_prompt(user_message: str, system_message: str | None = None) -> str:
    # Common eval/SFT prompt shape: system + user + open assistant turn.
    return render_messages(
        [
            {"role": "system", "content": system_message or DEFAULT_SYSTEM_PROMPT},
            {"role": "user", "content": user_message},
        ],
        add_generation_prompt=True,
    )


def render_tool_turn(result_block: str) -> str:
    # After a CALL executes, inject the tool RESULT and reopen assistant
    # generation so the model can continue the trace.
    return f"\n\n{render_message('tool', result_block)}\n\n<|im_start|>assistant\n"


def assert_glyph_template_parity(tokenizer: Any | None = None) -> None:
    """Hard-fail if the tokenizer chat template drifts from this renderer."""
    messages = [
        {"role": "system", "content": "SYS"},
        {"role": "user", "content": "USR"},
        {"role": "assistant", "content": 'CALL read_file(id="c1", file_path="x")\n<|im_end|>'},
        {"role": "tool", "content": "RESULT c1:\nstatus: success"},
    ]
    if tokenizer is not None:
        # Check the installed HuggingFace tokenizer template.
        rendered = tokenizer.apply_chat_template(
            messages, tokenize=False, add_generation_prompt=True
        )
    else:
        # Check the raw Jinja template string without needing a tokenizer.
        from jinja2.sandbox import ImmutableSandboxedEnvironment

        env = ImmutableSandboxedEnvironment(trim_blocks=True, lstrip_blocks=True)
        rendered = env.from_string(GLYPH_CHAT_TEMPLATE).render(
            messages=messages, add_generation_prompt=True
        )
    # Compare against the simple Python renderer above. These must match byte
    # for byte, or SFT/RL/eval will see different protocols.
    expected = render_messages(messages, add_generation_prompt=True)
    if rendered != expected:
        raise RuntimeError(
            "GLYPH_CHAT_TEMPLATE no longer matches the shared ChatML renderer.\n"
            f"rendered: {rendered!r}\n"
            f"expected: {expected!r}"
        )
