use std::collections::BTreeMap;

#[derive(Clone, Copy)]
enum Status {
    Open,
    InProgress,
    Closed,
}

impl Status {
    fn as_str(&self) -> &'static str {
        match self {
            Status::Open => "Open",
            Status::InProgress => "InProgress",
            Status::Closed => "Closed",
        }
    }
}

enum Event {
    Create(&'static str),
    Start(&'static str),
    Resolve(&'static str),
    Reopen(&'static str),
}

fn apply_event(state: &mut BTreeMap<&'static str, Status>, event: Event) {
    match event {
        Event::Create(id) => {
            state.insert(id, Status::Open);
        }
        Event::Start(id) => {
            if matches!(state.get(id), Some(Status::Open)) {
                state.insert(id, Status::InProgress);
            }
        }
        Event::Resolve(id) => {
            if matches!(state.get(id), Some(Status::Open | Status::InProgress)) {
                state.insert(id, Status::Closed);
            }
        }
        Event::Reopen(id) => {
            if state.contains_key(id) {
                state.insert(id, Status::Open);
            }
        }
    }
}

fn main() {
    let events = vec![
        Event::Create("T1"),
        Event::Start("T1"),
        Event::Resolve("T1"),
        Event::Reopen("T1"),
        Event::Resolve("T1"),
        Event::Create("T2"),
        Event::Start("T2"),
        Event::Reopen("T2"),
        Event::Create("T3"),
        Event::Reopen("T3"),
        Event::Resolve("T3"),
    ];

    let mut state = BTreeMap::new();
    for event in events {
        apply_event(&mut state, event);
    }

    for (id, status) in state {
        println!("{}={}", id, status.as_str());
    }
}
