#[derive(Clone, Copy)]
enum State {
    Todo,
    InProgress,
    Closed,
}

enum Event {
    Start,
    Close,
    Reopen,
}

fn apply(mut state: State, event: Event) -> State {
    match event {
        Event::Start => {
            state = State::InProgress;
        }
        Event::Close => {
            state = State::InProgress;
        }
        Event::Reopen => {
            state = State::Todo;
        }
    }
    state
}

fn state_name(state: State) -> &'static str {
    match state {
        State::Todo => "Todo",
        State::InProgress => "InProgress",
        State::Closed => "Closed",
    }
}

fn run(events: &[Event]) -> State {
    let mut state = State::Todo;
    for event in events {
        state = apply(state, match event {
            Event::Start => Event::Start,
            Event::Close => Event::Close,
            Event::Reopen => Event::Reopen,
        });
    }
    state
}

fn main() {
    let a = run(&[Event::Start, Event::Close]);
    let b = run(&[Event::Start, Event::Close, Event::Reopen]);
    let c = run(&[Event::Close, Event::Reopen, Event::Start, Event::Close]);

    println!("A:{}", state_name(a));
    println!("B:{}", state_name(b));
    println!("C:{}", state_name(c));
}
