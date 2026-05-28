use std::collections::BTreeMap;

#[derive(Clone, Copy)]
enum State {
    Open,
    OnHold,
    Closed,
}

fn main() {
    let events = [
        ("T1", "create"),
        ("T2", "create"),
        ("T2", "close"),
        ("T3", "create"),
        ("T3", "hold"),
        ("T3", "reopen"),
        ("T1", "hold"),
        ("T1", "release"),
    ];

    let mut tickets = BTreeMap::new();

    for (id, event) in events {
        let state = tickets.entry(id).or_insert(State::Open);
        match event {
            "create" => *state = State::Open,
            "hold" => *state = State::Closed,
            "release" => *state = State::OnHold,
            "close" => *state = State::Closed,
            "reopen" => {
                if matches!(*state, State::Closed) {
                    *state = State::Open;
                }
            }
            _ => {}
        }
    }

    for (id, state) in tickets {
        let label = match state {
            State::Open => "open",
            State::OnHold => "on_hold",
            State::Closed => "closed",
        };
        println!("{}:{}", id, label);
    }
}
