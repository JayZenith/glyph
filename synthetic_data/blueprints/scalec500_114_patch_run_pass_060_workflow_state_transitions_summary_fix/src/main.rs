use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Todo,
    Active,
    Blocked,
    Done,
    Canceled,
}

#[derive(Clone, Copy, Debug)]
enum Event {
    Start,
    Block,
    Unblock,
    Complete,
    Cancel,
    Reopen,
}

fn apply(state: State, event: Event) -> State {
    match (state, event) {
        (State::Todo, Event::Start) => State::Active,
        (State::Active, Event::Block) => State::Blocked,
        (State::Blocked, Event::Unblock) => State::Active,
        (_, Event::Complete) => State::Done,
        (_, Event::Cancel) => State::Canceled,
        (State::Done, Event::Reopen) => State::Todo,
        (s, _) => s,
    }
}

fn main() {
    let events = [
        ("A", Event::Start),
        ("A", Event::Complete),
        ("A", Event::Reopen),
        ("A", Event::Start),
        ("A", Event::Complete),
        ("B", Event::Start),
        ("B", Event::Cancel),
        ("B", Event::Reopen),
        ("B", Event::Start),
        ("B", Event::Cancel),
        ("C", Event::Start),
        ("C", Event::Block),
        ("C", Event::Complete),
    ];

    let mut tasks: BTreeMap<&str, State> = BTreeMap::new();
    for (id, event) in events {
        let state = tasks.get(id).copied().unwrap_or(State::Todo);
        tasks.insert(id, apply(state, event));
    }

    let mut active = 0;
    let mut done = 0;
    let mut blocked = 0;
    let mut canceled = 0;

    for (id, state) in &tasks {
        let label = match state {
            State::Todo => "todo",
            State::Active => {
                active += 1;
                "active"
            }
            State::Blocked => {
                blocked += 1;
                "blocked"
            }
            State::Done => {
                done += 1;
                "done"
            }
            State::Canceled => {
                canceled += 1;
                "canceled"
            }
        };
        println!("{id}: {label}");
    }

    println!(
        "active={active} done={done} blocked={blocked} canceled={canceled}"
    );
}
