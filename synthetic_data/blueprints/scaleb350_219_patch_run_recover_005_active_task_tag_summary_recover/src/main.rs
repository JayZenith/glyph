struct Task {
    name: &'static str,
    active: bool,
    priority: u8,
    tags: &'static [&'static str],
}

fn summarize(tasks: &[Task]) -> String {
    tasks
        .iter()
        .filter(|task| task.priority >= 2)
        .map(|task| {
            let tags = task
                .tags
                .iter()
                .copied()
                .filter(|tag| *tag != "ops")
                .collect::<Vec<_>>()
                .join(",");
            format!("{}:{}", task.name, tags)
        })
        .collect::<Vec<_>>()
        .join("|")
}

fn main() {
    let tasks = [
        Task {
            name: "alpha",
            active: true,
            priority: 3,
            tags: &["ops", "urgent"],
        },
        Task {
            name: "beta",
            active: false,
            priority: 4,
            tags: &["ops", "hold"],
        },
        Task {
            name: "gamma",
            active: true,
            priority: 2,
            tags: &["research", "ops"],
        },
        Task {
            name: "delta",
            active: true,
            priority: 5,
            tags: &["ops"],
        },
        Task {
            name: "epsilon",
            active: true,
            priority: 1,
            tags: &["urgent"],
        },
    ];

    println!("{}", summarize(&tasks));
}
