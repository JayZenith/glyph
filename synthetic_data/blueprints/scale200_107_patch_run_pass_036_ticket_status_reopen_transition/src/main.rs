use std::collections::BTreeMap;

#[derive(Clone, Copy)]
enum Status {
    Open,
    InProgress,
    Closed,
}

impl Status {
    fn as_str(self) -> &'static str {
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

fn apply_event(states: &mut BTreeMap<&'static str, Status>, event: Event) {
    match event {
        Event::Create(id) => {
            states.insert(id, Status::Open);
        }
        Event::Start(id) => {
            if let Some(state) = states.get_mut(id) {
                *state = Status::InProgress;
            }
        }
        Event::Resolve(id) => {
            if let Some(state) = states.get_mut(id) {
                *state = Status::Closed;
            }
        }
        Event::Reopen(id) => {
            if let Some(state) = states.get_mut(id) {
                *state = Status::Closed;
            }
        }
    }
}

fn main() {
    let events = vec![
        Event::Create("T1"),
        Event::Start("T1"),
        Event::Resolve("T1"),
        Event::Create("T2"),
        Event::Start("T2"),
        Event::Resolve("T2"),
        Event::Reopen("T2"),
        Event::Create("T3"),
        Event::Resolve("T3"),
    ];

    let mut states = BTreeMap::new();
    for event in events {
        apply_event(&mut states, event);
    }

    for (id, status) in states {
        println!("{}:{}", id, status.as_str());
    }
}
