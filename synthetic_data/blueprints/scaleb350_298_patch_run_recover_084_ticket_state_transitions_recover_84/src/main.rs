use std::collections::BTreeMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Open,
    InProgress,
    Closed,
}

impl State {
    fn as_str(self) -> &'static str {
        match self {
            State::Open => "open",
            State::InProgress => "in_progress",
            State::Closed => "closed",
        }
    }
}

enum Event<'a> {
    Create(&'a str),
    Start(&'a str),
    Resolve(&'a str),
    Reopen(&'a str),
}

fn apply(events: &[Event<'_>]) -> BTreeMap<String, State> {
    let mut states = BTreeMap::new();
    for event in events {
        match *event {
            Event::Create(id) => {
                states.insert(id.to_string(), State::Open);
            }
            Event::Start(id) => {
                if let Some(state) = states.get_mut(id) {
                    *state = State::InProgress;
                }
            }
            Event::Resolve(id) => {
                if let Some(state) = states.get_mut(id) {
                    *state = State::Closed;
                }
            }
            Event::Reopen(id) => {
                if let Some(state) = states.get_mut(id) {
                    *state = State::InProgress;
                }
            }
        }
    }
    states
}

fn main() {
    let events = [
        Event::Create("T1"),
        Event::Start("T1"),
        Event::Resolve("T1"),
        Event::Reopen("T1"),
        Event::Resolve("T1"),
        Event::Create("T2"),
        Event::Resolve("T2"),
        Event::Reopen("T2"),
        Event::Create("T3"),
        Event::Start("T3"),
        Event::Create("T3"),
    ];

    let states = apply(&events);
    for (id, state) in states {
        println!("{}:{}", id, state.as_str());
    }
}
