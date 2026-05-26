"""Dataset loading + tokenization with assistant-only loss masking."""
import hashlib
import json
from pathlib import Path
from datasets import Dataset

def load_traces(data_path: str) -> list[dict]:
    """Load processed traces from JSONL into [{text: ...}, ...]."""
    p = Path(data_path)
    if not p.exists():
        raise FileNotFoundError(
            f"{data_path} not found. Pass --data with a valid local dataset path."
        )
    traces = []
    with p.open() as f:
        for line in f:
            try:
                item = json.loads(line)
                traces.append({"text": item["trace"]})
            except Exception as e:
                raise ValueError(f"Failed to parse dataset line: {e}")
    print(f"Loaded {len(traces)} traces from {p}")
    return traces

def create_dataset(
    traces: list[dict],
    tokenizer,
    max_seq_length: int,
    cache_dir: str = ".cache",
) -> Dataset:
    """Tokenize traces with assistant-only loss masking and cache the result."""
    trace_fingerprint = hashlib.md5(
        "".join(trace["text"] for trace in traces).encode("utf-8")
    ).hexdigest()[:12]
    cache_key = hashlib.md5(
        f"v5_assistant_only_{len(traces)}_{trace_fingerprint}_{tokenizer.name_or_path}_{max_seq_length}".encode()
    ).hexdigest()[:12]
    cache_path = Path(cache_dir) / f"tokenized_{cache_key}"

    if cache_path.exists():
        print(f"✓ Loading tokenized dataset from cache: {cache_path}")
        dataset = Dataset.load_from_disk(str(cache_path))
        print(f"  Loaded {len(dataset)} samples")
        return dataset

    print(f"Tokenizing dataset (will cache to {cache_path})...")

    true_lengths = []
    for trace in traces:
        # dont cut seq even if exceeding model context length
        # don't add model-specific tokens BOS/EOS or chat formatting tokens
        # tokens results in { "input_ids": [...], "attention_mask": [...]}
        tokens = tokenizer(trace["text"], truncation=False, add_special_tokens=False)
        true_lengths.append(len(tokens["input_ids"]))

    over_lengths = [l for l in true_lengths if l > max_seq_length]
    if over_lengths:
        raise ValueError(
            f"{len(over_lengths)}/{len(traces)} traces exceed max_seq_length "
            f"({max_seq_length}). "
            f"Max length: {max(over_lengths)}"
        )
    else:
        print(f"✓ No truncation needed (max trace: {max(true_lengths)} tokens)")

    # Convert chat markers to IDs so masking code scans token sequences to
    # find assistant header, start unmasking labels skipping assistant to <|im_end|> then stops
    asst_header_ids = tokenizer.encode("<|im_start|>assistant\n", add_special_tokens=False)
    im_end_id = tokenizer.convert_tokens_to_ids("<|im_end|>")
    H = len(asst_header_ids)

    def make_labels(ids):
        labels = [-100] * len(ids)
        i = 0
        while i <= len(ids) - H:
            if ids[i:i + H] == asst_header_ids:
                j = i + H
                while j < len(ids) and ids[j] != im_end_id:
                    labels[j] = ids[j]
                    j += 1
                if j < len(ids):
                    labels[j] = ids[j]
                    i = j + 1
                else:
                    break
            else:
                i += 1
        return labels

    def tokenize(examples):
        tokenized = tokenizer(
            examples["text"],
            truncation=False,
            max_length=max_seq_length,
            padding=False,
            return_attention_mask=True,
        )
        tokenized["labels"] = [make_labels(ids) for ids in tokenized["input_ids"]]
        return tokenized

    # turn Python list like [{"text": "..."}, ...] into HF Dataset Object
    # Then dataset.map(...) applies tokenize() across dataset
    dataset = Dataset.from_list(traces)
    dataset = dataset.map(
        tokenize,
        batched=True,
        remove_columns=["text"],
        num_proc=4,
        desc="Tokenizing",
    )

    cache_path.parent.mkdir(parents=True, exist_ok=True)
    dataset.save_to_disk(str(cache_path))
    print(f"✓ Cached tokenized dataset to {cache_path}")

    return dataset
