use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Status {
    New,
    InProgress,
    Blocked,
    Closed,
}

impl Status {
    fn as_str(&self) -> &'static str {
        match self {
            Status::New => "New",
            Status::InProgress => "InProgress",
            Status::Blocked => "Blocked",
            Status::Closed => "Closed",
        }
    }
}

enum Event<'a> {
    Create(&'a str),
    Start(&'a str),
    Block(&'a str),
    Unblock(&'a str),
    Close(&'a str),
    Reopen(&'a str),
}

fn apply_event(state: &mut BTreeMap<String, Status>, event: Event<'_>) {
    match event {
        Event::Create(id) => {
            state.entry(id.to_string()).or_insert(Status::New);
        }
        Event::Start(id) => {
            if state.contains_key(id) {
                state.insert(id.to_string(), Status::InProgress);
            }
        }
        Event::Block(id) => {
            if state.contains_key(id) {
                state.insert(id.to_string(), Status::Blocked);
            }
        }
        Event::Unblock(id) => {
            if state.contains_key(id) {
                state.insert(id.to_string(), Status::New);
            }
        }
        Event::Close(id) => {
            if state.contains_key(id) {
                state.insert(id.to_string(), Status::Closed);
            }
        }
        Event::Reopen(id) => {
            if state.contains_key(id) {
                state.insert(id.to_string(), Status::InProgress);
            }
        }
    }
}

fn main() {
    let events = vec![
        Event::Create("A"),
        Event::Start("A"),
        Event::Block("A"),
        Event::Close("A"),
        Event::Unblock("A"),
        Event::Start("A"),
        Event::Close("A"),
        Event::Create("B"),
        Event::Block("B"),
        Event::Unblock("B"),
        Event::Start("B"),
        Event::Create("C"),
        Event::Close("C"),
        Event::Create("D"),
        Event::Start("D"),
        Event::Block("D"),
        Event::Reopen("D"),
        Event::Create("D"),
    ];

    let mut state = BTreeMap::new();
    for event in events {
        apply_event(&mut state, event);
    }

    let mut lines = Vec::new();
    for (id, status) in state {
        lines.push(format!("{}: {}", id, status.as_str()));
    }
    print!("{}", lines.join("\n"));
}
