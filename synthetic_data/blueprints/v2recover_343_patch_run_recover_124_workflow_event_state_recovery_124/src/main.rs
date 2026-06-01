use std::collections::BTreeMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Pending,
    Active,
    Closed,
}

fn main() {
    let events = [
        ("A", "create"),
        ("A", "start"),
        ("A", "close"),
        ("A", "start"),
        ("B", "create"),
        ("B", "start"),
        ("B", "start"),
        ("C", "create"),
        ("C", "close"),
    ];

    let mut items: BTreeMap<&str, State> = BTreeMap::new();
    let mut invalid_reopens = 0;
    let mut ignored_duplicates = 0;

    for (id, event) in events {
        let state = items.get(id).copied();
        match (state, event) {
            (None, "create") => {
                items.insert(id, State::Pending);
            }
            (Some(State::Pending), "start") => {
                items.insert(id, State::Active);
            }
            (Some(State::Active), "close") => {
                items.insert(id, State::Closed);
            }
            (Some(State::Closed), "start") => {
                items.insert(id, State::Active);
            }
            (Some(s), e) if (s == State::Pending || s == State::Active || s == State::Closed) && (e == "create" || e == "start" || e == "close") => {
                ignored_duplicates += 1;
            }
            _ => {}
        }
    }

    let mut pending = Vec::new();
    let mut active = Vec::new();
    let mut closed = Vec::new();

    for (id, state) in items {
        match state {
            State::Pending => pending.push(id),
            State::Active => active.push(id),
            State::Closed => closed.push(id),
        }
    }

    println!("closed: {}", active.join(","));
    println!("active: {}", closed.join(","));
    println!("pending: {}", if pending.is_empty() { "-".to_string() } else { pending.join(",") });
    println!("invalid_reopens: {}", invalid_reopens);
    println!("ignored_duplicates: {}", ignored_duplicates);
}
