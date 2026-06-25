from __future__ import annotations

import re
from dataclasses import dataclass


SEG_RE = re.compile(r"<\|im_start\|>(\w+)\n(.*?)<\|im_end\|>", re.DOTALL)
CALL_LINE_RE = re.compile(r"^\s*CALL\s+([A-Za-z_]\w*)\((.*)\)\s*$")
RESULT_ID_RE = re.compile(r"^\s*RESULT\s+([A-Za-z0-9_\-]+):", re.MULTILINE)
FINAL_RE = re.compile(r"^\s*FINAL:\s*", re.MULTILINE)
ROLE_LEAK_RE = re.compile(r"(<\|im_start\|>|<\|im_end\|>|^\s*(system|user|assistant|tool)\s*$)", re.MULTILINE)
REPETITION_RE = re.compile(r"(.{20,200}?)\1{4,}", re.DOTALL)
GIBBERISH_RE = re.compile(
    r"(\.waitKey|\.invokeLater|\.onreadystatechange|typealias|endphp|firebaseio|noreferrer|::::){8,}"
)
FINAL_MAX_CHARS = 512
CHATML_END = "<|im_end|>"


@dataclass
class ValidationResult:
    valid: bool
    errors: list[str]


@dataclass
class ProtocolCall:
    tool: str
    id: str
    params: dict[str, str]


def _joined_role_text(text: str, role: str) -> str:
    if "<|im_start|>" not in text:
        return text if role == "assistant" else ""
    return "\n".join(body for seg_role, body in SEG_RE.findall(text) if seg_role == role)


def assistant_text(text: str) -> str:
    return _joined_role_text(text, "assistant")


def tool_text(text: str) -> str:
    return _joined_role_text(text, "tool")


def strip_terminal_chatml_end(text: str) -> str:
    stripped = text.rstrip()
    if stripped.endswith(CHATML_END):
        stripped = stripped[: -len(CHATML_END)].rstrip()
    return stripped


def _parse_arg_blob(blob: str) -> tuple[dict[str, str], list[str]]:
    params: dict[str, str] = {}
    errors: list[str] = []
    pos = 0
    first = True
    item_re = re.compile(r'(\w+)\s*=\s*"((?:[^"\\]|\\.)*)"')
    while pos < len(blob):
        if not first:
            sep = re.match(r"\s*,\s*", blob[pos:])
            if not sep:
                errors.append("Malformed CALL argument separator")
                return params, errors
            pos += sep.end()
        first = False
        match = item_re.match(blob, pos)
        if not match:
            errors.append("Malformed CALL argument")
            return params, errors
        key, value = match.groups()
        if key in params:
            errors.append(f"Duplicate CALL argument: {key}")
        try:
            params[key] = bytes(value, "utf-8").decode("unicode_escape")
        except UnicodeDecodeError:
            errors.append(f"Invalid escape in CALL argument: {key}")
            params[key] = value
        pos = match.end()
    return params, errors


def parse_call_line(line: str) -> tuple[ProtocolCall | None, list[str]]:
    stripped = line.strip()
    if not stripped.startswith("CALL "):
        return None, []
    if "`" in stripped or stripped.endswith(";"):
        return None, ["Malformed CALL line terminator"]
    match = CALL_LINE_RE.fullmatch(stripped)
    if not match:
        return None, ["Malformed CALL line"]
    tool_name, arg_blob = match.groups()
    params, errors = _parse_arg_blob(arg_blob)
    call_id = params.pop("id", None)
    if not call_id:
        errors.append("Missing CALL id")
        call_id = ""
    if errors:
        return None, errors
    return ProtocolCall(tool=tool_name, id=call_id, params=params), []


def call_syntax_errors(text: str) -> list[str]:
    errors: list[str] = []
    for line_no, line in enumerate(text.splitlines(), 1):
        if not line.strip().startswith("CALL "):
            continue
        _, line_errors = parse_call_line(line)
        errors.extend(f"line {line_no}: {err}" for err in line_errors)
    return errors


def final_hygiene_errors(assistant_text: str) -> list[str]:
    errors: list[str] = []
    finals = [
        line.strip()
        for line in assistant_text.splitlines()
        if line.strip().startswith("FINAL:")
    ]
    if len(finals) != 1:
        return errors
    final_pos = assistant_text.rfind("FINAL:")
    tail = assistant_text[final_pos + len("FINAL:"):].strip()
    tail = re.sub(r"\s*<\|im_end\|>\s*$", "", tail).strip()
    body = tail
    if not body:
        errors.append("Empty FINAL")
    if "\n" in body or "\r" in body:
        errors.append("FINAL must be a single line")
    if len(body) > FINAL_MAX_CHARS:
        errors.append("FINAL too long")
    if any((ord(ch) < 32 and ch not in "\n\r\t") or ord(ch) > 126 for ch in body):
        errors.append("Non-ASCII FINAL")
    if "\ufffd" in body or "<|endoftext|>" in assistant_text:
        errors.append("Corrupt/generated special token in FINAL")
    if GIBBERISH_RE.search(body):
        errors.append("Gibberish FINAL")
    if re.search(r"\b[A-Z][A-Za-z]{7,}(?:Style|State|View|Manager)\b$", body):
        errors.append("Suspicious FINAL suffix")
    return errors


def parse_calls(text: str) -> list[ProtocolCall]:
    calls: list[ProtocolCall] = []
    for line in text.splitlines():
        call, errors = parse_call_line(line)
        if call is not None and not errors:
            calls.append(call)
    return calls


def extract_result_ids(text: str) -> list[str]:
    return RESULT_ID_RE.findall(text)


def extract_pending_call_ids(assistant_text: str, result_text: str = "") -> list[str]:
    call_ids = [call.id for call in parse_calls(assistant_text)]
    result_ids = set(extract_result_ids(result_text))
    return [call_id for call_id in call_ids if call_id not in result_ids]


def has_final(assistant_text: str) -> bool:
    return bool(FINAL_RE.search(assistant_text))


def final_count(assistant_text: str) -> int:
    return len(FINAL_RE.findall(assistant_text))


def ended_cleanly_after_final(assistant_text: str) -> bool:
    if not has_final(assistant_text):
        return False
    stripped = assistant_text.strip()
    last_final = stripped.rfind("FINAL:")
    if last_final < 0:
        return False
    tail = stripped[last_final:]
    return "CALL " not in tail[6:]


class SimpleTraceValidator:
    def validate(self, assistant_text: str, result_text: str = "") -> ValidationResult:
        errors: list[str] = []
        calls = parse_calls(assistant_text)
        call_ids = [call.id for call in calls]
        result_ids = extract_result_ids(result_text)

        if not has_final(assistant_text):
            errors.append("Missing response")
        if final_count(assistant_text) > 1:
            errors.append("Multiple final responses")
        if has_final(assistant_text) and not ended_cleanly_after_final(assistant_text):
            errors.append("Garbage after final response")
        if result_ids != call_ids[: len(result_ids)] or len(result_ids) < len(call_ids):
            errors.append("Tool calls without matching result")
        if REPETITION_RE.search(assistant_text):
            errors.append("Detected repetition")
        if GIBBERISH_RE.search(assistant_text) or "<|endoftext|>" in assistant_text:
            errors.append("Detected gibberish")
        errors.extend(call_syntax_errors(assistant_text))
        errors.extend(final_hygiene_errors(assistant_text))
        if ROLE_LEAK_RE.search(assistant_text):
            errors.append("Role marker leakage")
        return ValidationResult(valid=not errors, errors=errors)
