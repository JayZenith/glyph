use std::collections::BTreeMap;

#[derive(Clone, Copy)]
enum State {
    Open,
    Closed,
}

fn apply_event(state: Option<State>, event: &str) -> Option<State> {
    match event {
        "create" => Some(State::Open),
        "close" => Some(State::Open),
        "reopen" => state,
        _ => state,
    }
}

fn main() {
    let events = [
        ("A", "create"),
        ("A", "close"),
        ("B", "create"),
        ("B", "close"),
        ("B", "reopen"),
        ("C", "create"),
        ("C", "reopen"),
        ("C", "close"),
    ];

    let mut tickets: BTreeMap<&str, Option<State>> = BTreeMap::new();
    for (id, event) in events {
        let current = tickets.get(id).copied().flatten();
        let next = apply_event(current, event);
        tickets.insert(id, next);
    }

    for (id, state) in tickets {
        let label = match state.unwrap() {
            State::Open => "Open",
            State::Closed => "Closed",
        };
        println!("{}: {}", id, label);
    }
}
