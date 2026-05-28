use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Status {
    New,
    InProgress,
    Blocked,
    Resolved,
    Closed,
    Reopened,
}

#[derive(Clone, Copy, Debug)]
enum Event {
    Create,
    Start,
    Block,
    Unblock,
    Resolve,
    Close,
    Reopen,
}

fn apply_event(state: Option<Status>, event: Event) -> Option<Status> {
    match event {
        Event::Create => Some(Status::New),
        Event::Start => Some(Status::InProgress),
        Event::Block => Some(Status::Blocked),
        Event::Unblock => Some(Status::New),
        Event::Resolve => Some(Status::Resolved),
        Event::Close => Some(Status::Closed),
        Event::Reopen => Some(Status::New),
    }
}

fn status_name(s: Status) -> &'static str {
    match s {
        Status::New => "New",
        Status::InProgress => "InProgress",
        Status::Blocked => "Blocked",
        Status::Resolved => "Resolved",
        Status::Closed => "Closed",
        Status::Reopened => "Reopened",
    }
}

fn main() {
    let events = vec![
        ("T1", Event::Create),
        ("T1", Event::Start),
        ("T1", Event::Resolve),
        ("T1", Event::Close),
        ("T1", Event::Reopen),
        ("T1", Event::Resolve),
        ("T1", Event::Close),
        ("T2", Event::Create),
        ("T2", Event::Start),
        ("T2", Event::Block),
        ("T2", Event::Unblock),
        ("T2", Event::Resolve),
        ("T2", Event::Close),
        ("T2", Event::Reopen),
        ("T3", Event::Start),
        ("T3", Event::Create),
        ("T3", Event::Start),
        ("T4", Event::Create),
        ("T4", Event::Close),
        ("T4", Event::Start),
        ("T4", Event::Resolve),
        ("T4", Event::Close),
    ];

    let mut tickets: BTreeMap<&str, Option<Status>> = BTreeMap::new();
    for (id, event) in events {
        let current = tickets.get(id).copied().flatten();
        let next = apply_event(current, event);
        tickets.insert(id, next);
    }

    let mut out = Vec::new();
    for (id, state) in tickets {
        let name = state.map(status_name).unwrap_or("Missing");
        out.push(format!("{}:{}", id, name));
    }
    print!("{}", out.join("\n"));
}
