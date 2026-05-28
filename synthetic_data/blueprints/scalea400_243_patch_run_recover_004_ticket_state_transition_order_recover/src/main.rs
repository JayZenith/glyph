use std::collections::BTreeMap;

#[derive(Clone, Copy)]
enum Event {
    Open,
    Resolve,
    Reopen,
    Close,
}

fn apply_event(state: &str, event: Event) -> &'static str {
    match event {
        Event::Open => "open",
        Event::Resolve => "resolved",
        Event::Reopen => "resolved",
        Event::Close => state,
    }
}

fn main() {
    let input = [
        ("A", Event::Open),
        ("A", Event::Resolve),
        ("A", Event::Close),
        ("B", Event::Open),
        ("B", Event::Close),
        ("C", Event::Open),
        ("C", Event::Resolve),
        ("C", Event::Reopen),
    ];

    let mut states: BTreeMap<&str, &'static str> = BTreeMap::new();
    for (ticket, event) in input {
        let current = states.get(ticket).copied().unwrap_or("new");
        let next = apply_event(current, event);
        states.insert(ticket, next);
    }

    for (ticket, state) in states {
        println!("{}: {}", ticket, state);
    }
}
