enum TaskState {
    New,
    Running { retries: u8 },
    Waiting { dependency: &'static str },
    Failed { reason: &'static str, retryable: bool },
    Done,
}

struct Task {
    id: &'static str,
    state: TaskState,
}

fn describe(task: &Task) -> String {
    match &task.state {
        TaskState::New => format!("{} => queued", task.id),
        TaskState::Running { retries } => {
            if *retries == 0 {
                format!("{} => start job", task.id)
            } else {
                format!("{} => retrying ({})", task.id, retries)
            }
        }
        TaskState::Waiting { dependency } => format!("{} => blocked by {}", task.id, dependency),
        TaskState::Failed { reason, retryable } => {
            if *retryable {
                format!("{} => failed, will retry: {}", task.id, reason)
            } else {
                format!("{} => failed permanently: {}", task.id, reason)
            }
        }
        TaskState::Done => format!("{} => completed", task.id),
    }
}

fn main() {
    let tasks = [
        Task { id: "A1", state: TaskState::Running { retries: 0 } },
        Task { id: "B2", state: TaskState::Waiting { dependency: "dependency" } },
        Task { id: "C3", state: TaskState::Failed { reason: "missing token", retryable: true } },
        Task { id: "D4", state: TaskState::Done },
    ];

    for task in tasks.iter() {
        println!("{}", describe(task));
    }
}
