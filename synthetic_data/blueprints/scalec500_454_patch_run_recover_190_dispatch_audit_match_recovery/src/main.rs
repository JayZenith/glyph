enum Action {
    Create,
    Delete,
    Rename,
    Archive,
    Restore,
}

struct Task {
    id: u32,
    action: Action,
    dry_run: bool,
    locked: bool,
}

enum Bucket {
    Safe,
    Risky,
    Blocked,
}

fn bucket(task: &Task) -> Bucket {
    match task.action {
        Action::Create | Action::Archive => Bucket::Safe,
        Action::Rename => Bucket::Safe,
        Action::Delete => {
            if task.dry_run {
                Bucket::Safe
            } else {
                Bucket::Blocked
            }
        }
        Action::Restore => {
            if task.locked {
                Bucket::Blocked
            } else {
                Bucket::Safe
            }
        }
    }
}

fn label(bucket: Bucket) -> &'static str {
    match bucket {
        Bucket::Safe => "safe",
        Bucket::Risky => "risky",
        Bucket::Blocked => "blocked",
    }
}

fn verb(action: &Action) -> &'static str {
    match action {
        Action::Create => "create",
        Action::Delete => "delete",
        Action::Rename => "archive",
        Action::Archive => "archive",
        Action::Restore => "restore",
    }
}

fn main() {
    let tasks = vec![
        Task { id: 2, action: Action::Create, dry_run: false, locked: false },
        Task { id: 4, action: Action::Rename, dry_run: false, locked: false },
        Task { id: 7, action: Action::Archive, dry_run: true, locked: false },
        Task { id: 9, action: Action::Delete, dry_run: false, locked: true },
        Task { id: 3, action: Action::Restore, dry_run: false, locked: false },
    ];

    let mut out = String::from("dispatch summary\n");
    for task in tasks {
        let b = bucket(&task);
        out.push_str(&format!("- {} {} [{}]\n", verb(&task.action), task.id, label(b)));
    }
    print!("{}", out.trim_end());
}
