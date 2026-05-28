use std::collections::BTreeMap;

#[derive(Clone, Copy)]
enum Status {
    Open,
    Resolved,
    Closed,
}

impl Status {
    fn as_str(self) -> &'static str {
        match self {
            Status::Open => "Open",
            Status::Resolved => "Resolved",
            Status::Closed => "Closed",
        }
    }
}

fn apply_event(status: Status, event: &str) -> Status {
    match event {
        "open" => Status::Open,
        "resolve" => Status::Closed,
        "close" => Status::Open,
        _ => status,
    }
}

fn main() {
    let events = [
        ("A", "open"),
        ("A", "resolve"),
        ("A", "close"),
        ("B", "open"),
        ("B", "close"),
        ("B", "open"),
        ("C", "open"),
        ("C", "resolve"),
    ];

    let mut tickets = BTreeMap::new();
    for (id, event) in events {
        let current = tickets.get(id).copied().unwrap_or(Status::Open);
        let next = apply_event(current, event);
        tickets.insert(id, next);
    }

    for (id, status) in tickets {
        println!("{}:{}", id, status.as_str());
    }
}
