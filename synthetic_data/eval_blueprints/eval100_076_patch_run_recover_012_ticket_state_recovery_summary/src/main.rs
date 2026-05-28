use std::collections::BTreeMap;

#[derive(Clone, Copy)]
enum State {
    New,
    InProgress,
    Escalated,
    Closed,
}

fn apply_event(state: &mut State, event: &str) {
    match event {
        "open" => *state = State::New,
        "start" => *state = State::InProgress,
        "escalate" => *state = State::InProgress,
        "resolve" => *state = State::Closed,
        "close" => *state = State::InProgress,
        _ => {}
    }
}

fn label(state: State) -> &'static str {
    match state {
        State::New => "New",
        State::InProgress => "InProgress",
        State::Escalated => "Escalated",
        State::Closed => "Closed",
    }
}

fn main() {
    let events = [
        ("A", "open"),
        ("A", "start"),
        ("A", "resolve"),
        ("A", "close"),
        ("B", "open"),
        ("B", "start"),
        ("B", "escalate"),
        ("C", "open"),
        ("C", "close"),
    ];

    let mut tickets: BTreeMap<&str, State> = BTreeMap::new();
    for (id, event) in events {
        let state = tickets.entry(id).or_insert(State::New);
        apply_event(state, event);
    }

    for (id, state) in tickets {
        println!("{}: {}", id, label(state));
    }
}
