use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
    Cancelled,
}

#[derive(Clone, Copy, Debug)]
enum Event {
    Create,
    Start,
    Resolve,
    Close,
    Cancel,
    Reopen,
}

fn apply(status: Option<Status>, event: Event) -> Option<Status> {
    match (status, event) {
        (None, Event::Create) => Some(Status::Open),
        (Some(Status::Open), Event::Start) => Some(Status::InProgress),
        (Some(Status::InProgress), Event::Resolve) => Some(Status::Resolved),
        (Some(Status::Resolved), Event::Close) => Some(Status::Closed),
        (Some(Status::Open), Event::Cancel) | (Some(Status::InProgress), Event::Cancel) => Some(Status::Cancelled),
        (Some(Status::Resolved), Event::Reopen) | (Some(Status::Cancelled), Event::Reopen) => Some(Status::Open),
        (Some(s), _) => Some(s),
        (None, _) => None,
    }
}

fn status_name(status: Option<Status>) -> &'static str {
    match status {
        Some(Status::Open) => "Open",
        Some(Status::InProgress) => "InProgress",
        Some(Status::Resolved) => "Resolved",
        Some(Status::Closed) => "Closed",
        Some(Status::Cancelled) => "Cancelled",
        None => "Missing",
    }
}

fn main() {
    let events = [
        ("A", Event::Create),
        ("A", Event::Start),
        ("A", Event::Resolve),
        ("A", Event::Close),
        ("A", Event::Reopen),
        ("A", Event::Resolve),
        ("B", Event::Create),
        ("B", Event::Cancel),
        ("B", Event::Reopen),
        ("B", Event::Cancel),
        ("C", Event::Create),
        ("C", Event::Start),
        ("C", Event::Resolve),
        ("C", Event::Close),
        ("C", Event::Reopen),
        ("C", Event::Close),
    ];

    let mut tickets: BTreeMap<&str, Option<Status>> = BTreeMap::new();
    for (id, event) in events {
        let current = tickets.get(id).copied().unwrap_or(None);
        let next = apply(current, event);
        tickets.insert(id, next);
    }

    for (id, status) in tickets {
        println!("{}:{}", id, status_name(status));
    }
}
