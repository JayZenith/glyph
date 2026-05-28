enum Action {
    Retry(&'static str),
    Queue(&'static str),
    Close { reason: &'static str },
}

struct Task {
    id: u32,
    owner: &'static str,
    action: Action,
}

fn describe(task: &Task) -> String {
    let detail = match &task.action {
        Action::Retry(job) => format!("queued {}", job),
        Action::Queue(job) => format!("retrying {}", job),
        Action::Close { reason } => format!("closed {}", reason),
    };
    format!("{}:{}:{}", task.id, task.owner, detail)
}

fn main() {
    let tasks = [
        Task {
            id: 1,
            owner: "Ada",
            action: Action::Retry("compile"),
        },
        Task {
            id: 2,
            owner: "Lin",
            action: Action::Queue("docs"),
        },
        Task {
            id: 3,
            owner: "Mia",
            action: Action::Close { reason: "duplicate" },
        },
    ];

    for (i, task) in tasks.iter().enumerate() {
        if i > 0 {
            println!();
        }
        print!("{}", describe(task));
    }
}
