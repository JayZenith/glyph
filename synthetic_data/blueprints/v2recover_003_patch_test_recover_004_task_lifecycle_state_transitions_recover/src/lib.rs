#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Todo,
    InProgress,
    Done,
    Canceled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
    Complete,
    Cancel,
    Reopen,
}

pub fn advance(state: State, event: Event) -> State {
    match (state, event) {
        (State::Todo, Event::Start) => State::InProgress,
        (State::InProgress, Event::Complete) => State::Done,
        (_, Event::Cancel) => State::Canceled,
        (State::Done, Event::Reopen) => State::Todo,
        _ => state,
    }
}

pub fn apply_events(mut state: State, events: &[Event]) -> State {
    for &event in events {
        state = advance(state, event);
    }
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_flow_reaches_done() {
        let events = [Event::Start, Event::Complete];
        assert_eq!(apply_events(State::Todo, &events), State::Done);
    }

    #[test]
    fn done_task_cannot_be_canceled() {
        let events = [Event::Start, Event::Complete, Event::Cancel];
        assert_eq!(apply_events(State::Todo, &events), State::Done);
    }

    #[test]
    fn canceled_task_can_reopen_and_restart() {
        let events = [Event::Cancel, Event::Reopen, Event::Start];
        assert_eq!(apply_events(State::Todo, &events), State::InProgress);
    }
}
