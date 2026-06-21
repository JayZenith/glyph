# Crates 
The model never mutates the canonical crate. It is trained against a canonical path, while every RL rollout receives its own isolated Rust crate environment so concurrent rollouts cannot corrupt one another.

```bash
blueprint crate
       │
       ├── rollout 1 → crate_env_a/
       ├── rollout 2 → crate_env_b/
       └── rollout 3 → crate_env_c/
```

PRIME-RL -> verifiers -> RustToolEnv 
* trainer -> env/reward framework -> custom Rust world

PRIME-RL
* Generates rollouts, asks env what happens next, received reward, updates model

# Reward seperate from execution
- `RustToolEnv` does not assign reward, it just produces the env trajecotry. 
- `_rust_tool_reward` grades the trajectory after a rollout 


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

# FSDP mean Fully Sharded Data Parallel 
* Trainer splits model params, gradient,s, and optimizer states across trainer GPU.
* GPUs 0,1,2 are PRIME-RL managed, with 2 trainer GPUs + 1 inference GPU
* GPU 3 ran the frozen teacher 


# Teacher KL
During GRPO, trainer compared current policy's token distr against teacher dist and adds KL penalty so RL cannot drift far from SFT behaivor. 


# NCCL is GPU comms
Collectives are coordinated group comm: all-reduce, broadcast, reduce-scatter, all-gather. GPUs use to share param/gradient data to train sharded model.
