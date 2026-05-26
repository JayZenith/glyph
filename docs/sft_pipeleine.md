# Pipeline 
JSONL traces → data.py tokenization/masking → HuggingFace Trainer in train.py → saved checkpoint → eval_formal.py loads model → generation.py generates traces with mocked tool loops → scoring.py validates/scored outputs.

1. JSONL traces -> data.py tokenization/masking 

## Ex of JSONL trace
```bash
{"trace":"<|im_start|>system
You are a Rust coding agent. Use tools when needed. After FINAL, stop immediately.
<|im_end|>

<|im_start|>user
Fix the failing crate at /workspace/cases/addlib_bug. Read src/lib.rs, patch the bug, run cargo_test, then answer once.
<|im_end|>

<|im_start|>assistant
CALL read_file(file_path=\"/workspace/cases/addlib_bug/src/lib.rs\")\n<|im_end|>

<|im_start|>tool
RESULT read_file:
pub fn add(a: i32, b: i32) -> i32 { a - b }
<|im_end|>

<|im_start|>assistant
CALL apply_patch(file_path=\"/workspace/cases/addlib_bug/src/lib.rs\", find=\"a - b\", replace=\"a + b\")
<|im_end|>

<|im_start|>tool
RESULT apply_patch:\nstatus: success
<|im_end|>

<|im_start|>assistant
CALL cargo_test(project_path=\"/workspace/cases/addlib_bug\")
<|im_end|>

<|im_start|>tool
RESULT cargo_test:
status: success
stdout: test result: ok
<|im_end|>

<|im_start|>assistant
FINAL: Fixed the bug by changing subtraction to addition and verified it with cargo_test.
<|im_end|>"}
```

input_ids are the full tokenized trace. labels are the same shape but -100 means ignore in loss while actual token IDs mean to "predict the token". Cross-entropy loss is computed on labels not -100.

Masking also avoids wasting capacity predicting system/user/tool scaffolding. 

We don't patch entire dataset upfront as every sample would get padded to global maximum length and with batch size 1, padding is unnecessary.

we build cache fingerprint from dataset contents + tokenizer + masking mode + seq  length so retokenization is avoided. 

SUMMARY: data.py takes raw JSONL traces, tokenizes into input_ids, creates labels for training loss masking, builds HF dataset, removes raw "text" column, and caches processed dataset to disk for reuse.

HF Dataset sample is bassically:
```bash
{
    "input_ids": [...],
    "attention_mask": [...],
    "labels": [...]
}
``` 
with variable/unpadded sequence legnths. Padding happens later during batching via data collator. 

2. JSONL traces -> data.py tokenization/masking -> HF Trainer in train.py
When batching, shorter sequences get extended with pad token and here padding is set to <EOS> to match batch tensor sizes 

Gradient checkpointing trades comptue for VRAM. Instead of storing all intermediate activations during forward pass, recompute some during backward pass -> lower VRAM usage & slowe training. 

data_collator = DataCollatorForSeq2Seq(tokenizer=tokenizer, padding=True, pad_to_multiple_of=8)
- datasset already contains tokenized traces like:
* input_ids
* labels
* attention_mask
- collator gathers several samples, pads them, convert to batch tensors so GPU can train on them together


Gradient accumulation
* Not mainly due to optimizer stepping being expensive but VRAM! A true batch size of 8 means storing activations for 8 sequences simultaneously during backprop. Grad accumulation lets you process 1 sample at a time, accumualte gradients across 8 passes, and then apply one optimizer update using summed/average gradients. so optimizer update behaves similar to batch size 8 training, but memory usage is closer to batch size 1. 
* gradients accumulate in parameter .grad buffers over 8 samples and then after 8th mini-batch the optimizer.step() & optimizer.zero_grad() happens once. 

3. JSONL traces → data.py tokenization/masking → HuggingFace Trainer in train.py → saved checkpoint → eval_formal.py loads model → generation.py generates traces with mocked tool loops


# generation.py is inference orchestration for evals / does not train model

Model is SFT'd on traces shaped as: 
- assistant -> CALL
- tool -> RESULT
- assistant -> next step

So generation emits: CALL read_file(...)

and file detects unresoved call via: extract_pending_calls(...)

then injects:
```bash
<|im_start|>tool
RESULT read_file:
status: success
<|im_end|>

<|im_start|>assistant
```

using inject_mock_result(...) and generation resumes from updated prompt.


4. prompt_loader.py loading YAML prompts + scoring.py validating/scored outputs
