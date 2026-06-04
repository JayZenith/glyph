# Crates 
Model never touches real crate, its trained on a canonical path for simplicty but during RL, every rollout gets its own private sandbox copy of the crate. 

```bash
blueprint crate
       │
       ├── rollout 1 → sandbox_a/
       ├── rollout 2 → sandbox_b/
       └── rollout 3 → sandbox_c/
```

This prevents rollouts from corrupting each other. 



PRIME-RL -> verifiers -> RustToolEnv 
* trainer -> env/reward framework -> custom Rust world

PRIME-RL
* Generates rollouts, asks env what happens next, received reward, updates model


# Reward seperate from execution
- `RustToolEnv` does not assign reward, it just produces the env trajecotry. 
- `_rust_tool_reward` grades teh trajectory after a rollout 



# Naive RL loop vs asynchronous RL loop
- Naive generates rollouts -> computes rewards -> Train -> generates rollouts 
- PRIME-RL says Trainer, Inference, Orchestrator all run seperately 
  - Infernce generates samples
  - Orchestrator asks inference for rollouts (e.g. give 16 samples for prompt x) then runs the environment (RustToolEnv subclassing vf.MultiTurnEnv) and then scores the rollout and builds training batches.
    - So orchestrator grades trajectories 
  - Trainer recevies prompt, rollout, reward, advantage and performs GRPO updates. 
* So think of Policy model we run infenrece on, trainer training it, an teacher model used as the KL anchor 


# How GRPO works 
- increses probability of actions in positive-advantage rollotus
- Decreases those in negative-advantage rollouts
