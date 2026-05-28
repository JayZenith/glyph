use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Status {
    New,
    InProgress,
    Resolved,
    Closed,
}

impl Status {
    fn as_str(self) -> &'static str {
        match self {
            Status::New => "new",
            Status::InProgress => "in_progress",
            Status::Resolved => "resolved",
            Status::Closed => "closed",
        }
    }
}

fn apply(status: Status, event: &str) -> Status {
    match event {
        "create" => Status::New,
        "start" => match status {
            Status::New => Status::InProgress,
            s => s,
        },
        "resolve" => match status {
            Status::InProgress => Status::Resolved,
            s => s,
        },
        "close" => match status {
            Status::Resolved => Status::Closed,
            s => s,
        },
        "reopen" => match status {
            Status::Resolved | Status::Closed => Status::InProgress,
            s => s,
        },
        _ => status,
    }
}

fn main() {
    let events = [
        ("A", "create"),
        ("A", "start"),
        ("A", "resolve"),
        ("A", "close"),
        ("A", "reopen"),
        ("A", "resolve"),
        ("B", "create"),
        ("B", "start"),
        ("B", "resolve"),
        ("B", "close"),
        ("B", "resolve"),
        ("C", "resolve"),
        ("C", "create"),
        ("C", "start"),
        ("C", "resolve"),
        ("D", "create"),
        ("D", "start"),
        ("D", "close"),
        ("D", "reopen"),
    ];

    let mut tickets: BTreeMap<&str, Status> = BTreeMap::new();

    for (id, event) in events {
        let current = tickets.get(id).copied().unwrap_or(Status::New);
        let next = if !tickets.contains_key(id) && event != "create" {
            current
        } else {
            apply(current, event)
        };
        tickets.insert(id, next);
    }

    let mut out = Vec::new();
    for (id, status) in tickets {
        out.push(format!("{}: {}", id, status.as_str()));
    }
    print!("{}", out.join("\n"));
}
