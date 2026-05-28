use std::collections::BTreeMap;

#[derive(Clone, Copy)]
enum Event {
    Open,
    Activate,
    Disable,
    Close,
    Reopen,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    New,
    Open,
    Active,
    Disabled,
    Closed,
}

fn apply(mut state: State, event: Event) -> State {
    match event {
        Event::Open => {
            if state == State::New {
                state = State::Open;
            }
        }
        Event::Activate => {
            if matches!(state, State::Open | State::Disabled) {
                state = State::Active;
            }
        }
        Event::Disable => {
            if state == State::Active {
                state = State::Disabled;
            }
        }
        Event::Close => {
            if state != State::Closed {
                state = State::Closed;
            }
        }
        Event::Reopen => {
            if state == State::Closed {
                state = State::Open;
            }
        }
    }
    state
}

fn label(state: State) -> &'static str {
    match state {
        State::New => "new",
        State::Open => "open",
        State::Active => "active",
        State::Disabled => "disabled",
        State::Closed => "closed",
    }
}

fn main() {
    let events = vec![
        ("A1", Event::Open),
        ("A1", Event::Activate),
        ("A1", Event::Disable),
        ("A1", Event::Activate),
        ("B2", Event::Close),
        ("B2", Event::Reopen),
        ("B2", Event::Activate),
        ("B2", Event::Disable),
        ("C3", Event::Activate),
        ("C3", Event::Open),
        ("C3", Event::Close),
        ("C3", Event::Reopen),
        ("C3", Event::Close),
        ("D4", Event::Open),
        ("D4", Event::Close),
    ];

    let mut states: BTreeMap<&str, State> = BTreeMap::new();
    for (id, event) in events {
        let current = states.get(id).copied().unwrap_or(State::New);
        let next = apply(current, event);
        states.insert(id, next);
    }

    for (id, state) in states {
        println!("{}={}", id, label(state));
    }
}
