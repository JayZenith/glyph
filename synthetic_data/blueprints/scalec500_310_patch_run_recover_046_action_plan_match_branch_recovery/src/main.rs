#[derive(Clone, Copy)]
enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

enum Task {
    Review { area: &'static str },
    Build { target: &'static str, release: bool },
    Deploy { service: &'static str, region_count: u8 },
    Audit { system: &'static str, auto: bool },
    Incident { service: &'static str, acknowledged: bool },
}

fn priority_label(p: Priority) -> &'static str {
    match p {
        Priority::Low => "low",
        Priority::Normal => "normal",
        Priority::High => "high",
        Priority::Critical => "critical",
    }
}

fn plan(task: &Task, priority: Priority) -> String {
    match task {
        Task::Review { area } => format!("review {area} => skim later [{}]", priority_label(priority)),
        Task::Build { target, release } => {
            let step = if *release { "compile standard" } else { "run fast checks" };
            let label = match priority {
                Priority::Critical => "critical",
                _ => priority_label(priority),
            };
            format!("build {target} => {step} [{label}]")
        }
        Task::Deploy {
            service,
            region_count,
        } => {
            let step = if *region_count > 1 {
                "deploy immediately"
            } else {
                "schedule canary"
            };
            let label = if *region_count > 2 {
                "critical"
            } else {
                priority_label(priority)
            };
            format!("deploy {service} => {step} [{label}]")
        }
        Task::Audit { system, auto } => {
            let step = if *auto { "investigate manually" } else { "collect evidence" };
            format!("audit {system} => {step} [{}]", priority_label(priority))
        }
        Task::Incident {
            service,
            acknowledged,
        } => {
            let step = if *acknowledged { "page on-call" } else { "open ticket" };
            let label = if *acknowledged {
                priority_label(priority)
            } else {
                "normal"
            };
            format!("incident {service} => {step} [{label}]")
        }
    }
}

fn main() {
    let jobs = [
        (Task::Review { area: "docs" }, Priority::Normal),
        (
            Task::Build {
                target: "release",
                release: true,
            },
            Priority::High,
        ),
        (
            Task::Deploy {
                service: "edge",
                region_count: 1,
            },
            Priority::High,
        ),
        (
            Task::Deploy {
                service: "core",
                region_count: 3,
            },
            Priority::High,
        ),
        (
            Task::Audit {
                system: "billing",
                auto: false,
            },
            Priority::Normal,
        ),
        (
            Task::Incident {
                service: "api",
                acknowledged: true,
            },
            Priority::Critical,
        ),
        (
            Task::Incident {
                service: "search",
                acknowledged: false,
            },
            Priority::High,
        ),
    ];

    for (task, priority) in jobs {
        println!("{}", plan(&task, priority));
    }
}
