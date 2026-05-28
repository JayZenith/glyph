use std::collections::BTreeMap;

#[derive(Clone, Copy)]
enum State {
    Open,
    Closed,
    Reopened,
}

impl State {
    fn as_str(self) -> &'static str {
        match self {
            State::Open => "Open",
            State::Closed => "Closed",
            State::Reopened => "Reopened",
        }
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
    ];

    let mut states: BTreeMap<&str, State> = BTreeMap::new();

    for (id, action) in events {
        let next = match action {
            "create" => Some(State::Open),
            "close" => Some(State::Open),
            "reopen" => Some(State::Open),
            _ => None,
        };

        if let Some(state) = next {
            states.insert(id, state);
        }
    }

    for (id, state) in states {
        println!("{}: {}", id, state.as_str());
    }
}
