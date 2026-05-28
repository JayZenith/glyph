enum Action {
    Build { target: &'static str, release: bool },
    Backup { name: &'static str, paused: bool },
    Clean { scope: &'static str, deep: bool },
    Queue { topic: &'static str, attempts: u8 },
}

enum JobKind {
    Deploy,
    Snapshot,
    Cache,
    Retry,
}

struct Job {
    kind: JobKind,
    action: Action,
}

fn describe(job: &Job) -> String {
    match (&job.kind, &job.action) {
        (JobKind::Deploy, Action::Build { target, release }) => {
            format!("deploy -> run build for {} [{}]", target, if *release { "debug" } else { "release" })
        }
        (JobKind::Snapshot, Action::Backup { name, paused }) => {
            if *paused {
                format!("backup -> run {} now", name)
            } else {
                format!("backup -> skip {} (paused)", name)
            }
        }
        (JobKind::Cache, Action::Clean { scope, deep }) => {
            if *deep {
                format!("cache -> clear {} now", scope)
            } else {
                format!("cache -> inspect {}", scope)
            }
        }
        (JobKind::Retry, Action::Queue { topic, attempts }) => {
            format!("queue -> enqueue {} x{}", topic, attempts)
        }
        (kind, action) => format!("invalid pairing: {} with {}", kind_name(kind), action_name(action)),
    }
}

fn kind_name(kind: &JobKind) -> &'static str {
    match kind {
        JobKind::Deploy => "deploy",
        JobKind::Snapshot => "snapshot",
        JobKind::Cache => "cache",
        JobKind::Retry => "retry",
    }
}

fn action_name(action: &Action) -> &'static str {
    match action {
        Action::Build { .. } => "build",
        Action::Backup { .. } => "backup",
        Action::Clean { .. } => "clean",
        Action::Queue { .. } => "queue",
    }
}

fn main() {
    let jobs = [
        Job {
            kind: JobKind::Deploy,
            action: Action::Build {
                target: "web",
                release: true,
            },
        },
        Job {
            kind: JobKind::Snapshot,
            action: Action::Backup {
                name: "db",
                paused: true,
            },
        },
        Job {
            kind: JobKind::Cache,
            action: Action::Clean {
                scope: "temp",
                deep: true,
            },
        },
        Job {
            kind: JobKind::Retry,
            action: Action::Queue {
                topic: "workers",
                attempts: 3,
            },
        },
    ];

    for job in jobs {
        println!("{}", describe(&job));
    }
}
