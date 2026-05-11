from __future__ import annotations

from datasets import Dataset

import verifiers as vf
from core.validator import TaskValidator
from task_format import TaskVerifier, load_prompts


async def _verifier_reward(completion, **kwargs) -> float:
    verifier: TaskVerifier = kwargs["verifier"]
    text = completion if isinstance(completion, str) else completion[-1]["content"]
    score, _ = verifier.compute_reward(text)
    return score


async def _smoke_reward(completion, **kwargs) -> float:
    verifier: TaskVerifier = kwargs["verifier"]
    validator: TaskValidator = kwargs["validator"]
    text = completion if isinstance(completion, str) else completion[-1]["content"]
    struct_pass = 1.0 if validator.validate(text).valid else 0.0
    section_credit, _ = verifier.compute_reward(text)
    return struct_pass + section_credit


def load_environment(
    data_path: str = "traces.processed.jsonl",
    max_samples: int | None = None,
    max_trace_chars: int | None = 50000,
    env_id: str = "task-trace",
    reward_mode: str = "verifier_only",
) -> vf.Environment:
    """Load a TASK-format verifiers environment backed by local prompts."""

    prompts, _ = load_prompts(
        data_path=data_path,
        max_samples=max_samples,
        max_trace_chars=max_trace_chars,
    )
    dataset = Dataset.from_list(
        [
            {
                "prompt": item["prompt"],
                "task": env_id,
            }
            for item in prompts
        ]
    )

    parser = vf.Parser()
    verifier = TaskVerifier()
    validator = TaskValidator()
    rubric = vf.Rubric(parser=parser)
    rubric.class_objects["verifier"] = verifier
    rubric.class_objects["validator"] = validator
    if reward_mode == "smoke_deterministic":
        rubric.add_reward_func(_smoke_reward, weight=1.0)
    elif reward_mode == "verifier_only":
        rubric.add_reward_func(_verifier_reward, weight=1.0)
    else:
        raise ValueError(f"Unknown reward_mode={reward_mode!r}")

    env = vf.SingleTurnEnv(
        dataset=dataset,
        parser=parser,
        rubric=rubric,
        message_type="completion",
        env_id=env_id,
    )
    return env
