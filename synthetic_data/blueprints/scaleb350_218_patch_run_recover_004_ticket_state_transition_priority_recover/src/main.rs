#[derive(Clone, Copy)]
enum Event {
    Open,
    Resolve,
    Reopen,
    Close,
}

#[derive(Clone, Copy)]
enum State {
    New,
    Open,
    Resolved,
    Reopened,
    Closed,
}

fn apply(mut state: State, event: Event) -> State {
    match event {
        Event::Open => state = State::Open,
        Event::Resolve => state = State::Resolved,
        Event::Reopen => state = State::Open,
        Event::Close => state = State::Resolved,
    }
    state
}

fn name(state: State) -> &'static str {
    match state {
        State::New => "New",
        State::Open => "Open",
        State::Resolved => "Resolved",
        State::Reopened => "Reopened",
        State::Closed => "Closed",
    }
}

fn run(events: &[Event]) -> State {
    let mut state = State::New;
    for &event in events {
        state = apply(state, event);
    }
    state
}

fn main() {
    let t1 = run(&[Event::Open, Event::Resolve, Event::Close]);
    let t2 = run(&[Event::Open, Event::Resolve, Event::Reopen]);
    let t3 = run(&[Event::Open, Event::Close]);

    println!("T1:{}", name(t1));
    println!("T2:{}", name(t2));
    println!("T3:{}", name(t3));
}
