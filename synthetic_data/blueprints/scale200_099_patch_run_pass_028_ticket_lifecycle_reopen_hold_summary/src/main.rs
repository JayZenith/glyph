use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Open,
    InProgress,
    OnHold,
    Closed,
}

#[derive(Clone, Copy, Debug)]
enum Event {
    Open,
    Start,
    Hold,
    Resume,
    Resolve,
    Reopen,
}

fn apply(state: Option<State>, event: Event) -> Option<State> {
    match (state, event) {
        (None, Event::Open) => Some(State::Open),
        (Some(State::Open), Event::Start) => Some(State::InProgress),
        (Some(State::InProgress), Event::Hold) => Some(State::OnHold),
        (Some(State::OnHold), Event::Resume) => Some(State::InProgress),
        (Some(State::Open), Event::Resolve) | (Some(State::InProgress), Event::Resolve) => Some(State::Closed),
        (Some(State::Closed), Event::Reopen) => Some(State::InProgress),
        _ => state,
    }
}

fn label(state: Option<State>) -> &'static str {
    match state.unwrap_or(State::Open) {
        State::Open => "Open",
        State::InProgress => "InProgress",
        State::OnHold => "OnHold",
        State::Closed => "Closed",
    }
}

fn main() {
    let events = [
        ("A", Event::Open),
        ("A", Event::Start),
        ("A", Event::Resolve),
        ("B", Event::Open),
        ("B", Event::Start),
        ("B", Event::Hold),
        ("B", Event::Resume),
        ("B", Event::Resolve),
        ("B", Event::Reopen),
        ("C", Event::Open),
        ("C", Event::Resolve),
        ("D", Event::Open),
        ("D", Event::Hold),
        ("D", Event::Start),
        ("D", Event::Resolve),
    ];

    let mut states: BTreeMap<&str, Option<State>> = BTreeMap::new();
    for (id, event) in events {
        let current = states.get(id).copied().flatten();
        let next = apply(current, event);
        states.insert(id, next);
    }

    for (id, state) in states {
        println!("{}:{}", id, label(state));
    }
}
