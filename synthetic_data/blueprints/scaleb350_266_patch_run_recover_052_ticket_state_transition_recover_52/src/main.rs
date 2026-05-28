use std::collections::BTreeMap;

#[derive(Clone, Copy)]
enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

impl Status {
    fn as_str(&self) -> &'static str {
        match self {
            Status::Open => "Open",
            Status::InProgress => "InProgress",
            Status::Resolved => "Resolved",
            Status::Closed => "Closed",
        }
    }
}

fn main() {
    let events = [
        ("T1", "start"),
        ("T1", "resolve"),
        ("T1", "close"),
        ("T2", "start"),
        ("T2", "resolve"),
        ("T3", "close"),
    ];

    let mut states: BTreeMap<&str, Status> = BTreeMap::new();

    for (id, event) in events {
        let next = match event {
            "start" => Status::InProgress,
            "resolve" => Status::Closed,
            "close" => Status::Closed,
            _ => continue,
        };
        states.insert(id, next);
    }

    for (id, status) in states {
        println!("{}:{}", id, status.as_str());
    }
}
