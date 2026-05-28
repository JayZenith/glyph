enum Task {
    Run { name: &'static str },
    Skip { name: &'static str, reason: SkipReason },
}

enum SkipReason {
    Disabled,
    Manual,
    DryRun,
}

fn render(task: &Task) -> String {
    match task {
        Task::Run { name } => format!("{name}:run"),
        Task::Skip { name, reason } => match reason {
            SkipReason::Disabled => format!("{name}:skip(disabled)"),
            SkipReason::Manual => format!("{name}:skip(disabled)"),
            SkipReason::DryRun => format!("{name}:skip(dry-run)"),
        },
    }
}

fn main() {
    let tasks = [
        Task::Run {
            name: "deploy-service",
        },
        Task::Skip {
            name: "clean-cache",
            reason: SkipReason::Manual,
        },
        Task::Skip {
            name: "backup-db",
            reason: SkipReason::Disabled,
        },
        Task::Skip {
            name: "audit-logs",
            reason: SkipReason::DryRun,
        },
    ];

    for (i, task) in tasks.iter().enumerate() {
        if i > 0 {
            println!();
        }
        print!("{}", render(task));
    }
}
