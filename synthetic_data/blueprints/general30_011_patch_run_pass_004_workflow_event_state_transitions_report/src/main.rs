use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Open,
    InReview,
    Closed,
    Reopened,
}

#[derive(Clone, Copy, Debug)]
enum Event {
    Create,
    Review,
    Approve,
    Reject,
    Reopen,
    Close,
}

fn apply_event(state: Option<State>, event: Event) -> Option<State> {
    match event {
        Event::Create => Some(State::Open),
        Event::Review => Some(State::InReview),
        Event::Approve => Some(State::Closed),
        Event::Reject => Some(State::Open),
        Event::Reopen => Some(State::Open),
        Event::Close => Some(State::Closed),
    }
}

fn state_name(state: State) -> &'static str {
    match state {
        State::Open => "open",
        State::InReview => "in_review",
        State::Closed => "closed",
        State::Reopened => "reopened",
    }
}

fn main() {
    let events = [
        ("A", Event::Create),
        ("A", Event::Review),
        ("A", Event::Approve),
        ("A", Event::Reopen),
        ("A", Event::Close),
        ("B", Event::Create),
        ("B", Event::Review),
        ("B", Event::Reject),
        ("B", Event::Review),
        ("B", Event::Approve),
        ("B", Event::Reopen),
        ("C", Event::Review),
        ("C", Event::Create),
        ("D", Event::Create),
        ("D", Event::Close),
    ];

    let mut items: BTreeMap<&str, State> = BTreeMap::new();

    for (id, event) in events {
        let next = apply_event(items.get(id).copied(), event);
        if let Some(state) = next {
            items.insert(id, state);
        }
    }

    for (id, state) in items {
        println!("{}: {}", id, state_name(state));
    }
}
