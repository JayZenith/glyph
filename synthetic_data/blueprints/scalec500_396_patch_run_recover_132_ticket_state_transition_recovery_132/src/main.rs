use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Status {
    Open,
    Closed,
    Cancelled,
}

impl Status {
    fn as_str(self) -> &'static str {
        match self {
            Status::Open => "Open",
            Status::Closed => "Closed",
            Status::Cancelled => "Cancelled",
        }
    }
}

fn apply_event(state: Option<Status>, event: &str) -> Option<Status> {
    match event {
        "open" => Some(Status::Open),
        "close" => Some(Status::Closed),
        "reopen" => Some(Status::Open),
        "cancel" => Some(Status::Cancelled),
        _ => state,
    }
}

fn main() {
    let events = [
        ("A", "open"),
        ("A", "close"),
        ("A", "reopen"),
        ("A", "close"),
        ("B", "cancel"),
        ("B", "reopen"),
        ("C", "close"),
        ("C", "open"),
        ("C", "open"),
    ];

    let mut tickets: BTreeMap<&str, Option<Status>> = BTreeMap::new();
    for (id, event) in events {
        let current = tickets.get(id).copied().flatten();
        let next = apply_event(current, event);
        tickets.insert(id, next);
    }

    let mut lines = Vec::new();
    for (id, state) in tickets {
        if let Some(status) = state {
            lines.push(format!("{}:{}", id, status.as_str()));
        }
    }
    print!("{}", lines.join("\n"));
}
