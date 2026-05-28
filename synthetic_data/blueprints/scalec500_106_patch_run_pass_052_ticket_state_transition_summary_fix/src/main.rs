use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Open,
    InProgress,
    Blocked,
    Closed,
}

#[derive(Clone, Copy, Debug)]
enum Event {
    Create,
    Start,
    Block,
    Unblock,
    Resolve,
    Reopen,
}

fn apply_event(state: Option<State>, event: Event) -> Option<State> {
    match event {
        Event::Create => Some(State::Open),
        Event::Start => Some(State::InProgress),
        Event::Block => Some(State::Blocked),
        Event::Unblock => Some(State::Open),
        Event::Resolve => Some(State::Closed),
        Event::Reopen => Some(State::Open),
    }
}

fn main() {
    let events = [
        ("T1", Event::Create),
        ("T1", Event::Start),
        ("T1", Event::Resolve),
        ("T1", Event::Reopen),
        ("T1", Event::Start),
        ("T1", Event::Resolve),
        ("T2", Event::Create),
        ("T2", Event::Start),
        ("T2", Event::Block),
        ("T2", Event::Unblock),
        ("T2", Event::Start),
        ("T3", Event::Start),
        ("T3", Event::Create),
    ];

    let mut tickets: BTreeMap<&str, State> = BTreeMap::new();
    for (id, event) in events {
        let current = tickets.get(id).copied();
        if let Some(next) = apply_event(current, event) {
            tickets.insert(id, next);
        }
    }

    let mut lines = Vec::new();
    for (id, state) in tickets {
        lines.push(format!("{}: {:?}", id, state));
    }
    println!("{}", lines.join("\n"));
}
