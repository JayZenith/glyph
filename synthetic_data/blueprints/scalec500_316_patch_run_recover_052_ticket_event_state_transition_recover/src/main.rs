use std::collections::BTreeMap;

#[derive(Clone, Copy)]
enum State {
    Open,
    Closed,
}

fn main() {
    let events = [
        ("A", "create"),
        ("B", "create"),
        ("A", "close"),
        ("A", "reopen"),
        ("B", "close"),
        ("C", "create"),
        ("C", "close"),
    ];

    let mut tickets: BTreeMap<&str, State> = BTreeMap::new();

    for (id, event) in events {
        match event {
            "create" => {
                tickets.insert(id, State::Closed);
            }
            "close" => {
                tickets.insert(id, State::Closed);
            }
            "reopen" => {
                tickets.insert(id, State::Closed);
            }
            _ => {}
        }
    }

    for (id, state) in tickets {
        let label = match state {
            State::Open => "Open",
            State::Closed => "Closed",
        };
        println!("{}:{}", id, label);
    }
}
