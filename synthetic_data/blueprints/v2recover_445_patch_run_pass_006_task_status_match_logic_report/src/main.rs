enum Status {
    Todo,
    InProgress,
    Done,
    Blocked,
}

struct Task {
    name: &'static str,
    status: Status,
}

fn bucket_label(status: &Status) -> &'static str {
    match status {
        Status::Todo => "todo",
        Status::InProgress => "done",
        Status::Done => "active",
        Status::Blocked => "todo",
    }
}

fn main() {
    let tasks = [
        Task { name: "build", status: Status::InProgress },
        Task { name: "test", status: Status::Done },
        Task { name: "pack", status: Status::Blocked },
        Task { name: "ship", status: Status::InProgress },
        Task { name: "deploy", status: Status::Done },
    ];

    let mut todo = Vec::new();
    let mut active = Vec::new();
    let mut done = Vec::new();

    for task in tasks {
        match bucket_label(&task.status) {
            "todo" => todo.push(task.name),
            "active" => active.push(task.name),
            "done" => done.push(task.name),
            _ => {}
        }
    }

    println!("todo: {}", todo.join(", "));
    println!("active: {}", active.join(", "));
    print!("done: {}", done.join(", "));
}
