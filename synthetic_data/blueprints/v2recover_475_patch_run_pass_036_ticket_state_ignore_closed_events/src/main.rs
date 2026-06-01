#[derive(Clone, Copy)]
enum Event {
    Create,
    Start,
    Resolve,
    Reopen,
}

#[derive(Clone, Copy)]
enum State {
    Missing,
    Open,
    InProgress,
    Closed,
}

impl State {
    fn apply(self, event: Event) -> State {
        match (self, event) {
            (State::Missing, Event::Create) => State::Open,
            (State::Open, Event::Start) => State::InProgress,
            (State::InProgress, Event::Resolve) => State::Closed,
            (State::Closed, Event::Reopen) => State::Open,
            (_, Event::Create) => self,
            (_, Event::Start) => State::InProgress,
            (_, Event::Resolve) => State::Closed,
            (_, Event::Reopen) => State::Open,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            State::Missing => "Missing",
            State::Open => "Open",
            State::InProgress => "In Progress",
            State::Closed => "Closed",
        }
    }
}

fn run(events: &[Event]) -> State {
    let mut state = State::Missing;
    for &event in events {
        state = state.apply(event);
    }
    state
}

fn main() {
    let a = run(&[Event::Create, Event::Start, Event::Resolve, Event::Start]);
    let b = run(&[Event::Create, Event::Start]);
    let c = run(&[Event::Reopen, Event::Create]);

    println!("A: {}", a.as_str());
    println!("B: {}", b.as_str());
    println!("C: {}", c.as_str());
}
