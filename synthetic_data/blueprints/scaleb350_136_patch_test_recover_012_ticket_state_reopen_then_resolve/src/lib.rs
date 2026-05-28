#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Start,
    Resolve,
    Close,
    Reopen,
}

pub fn apply_event(state: State, event: Event) -> State {
    match (state, event) {
        (State::Open, Event::Start) => State::InProgress,
        (State::InProgress, Event::Resolve) => State::Resolved,
        (State::Resolved, Event::Close) => State::Closed,
        (State::Resolved, Event::Reopen) => State::Open,
        (State::Closed, Event::Reopen) => State::Open,
        _ => state,
    }
}

pub fn apply_events(mut state: State, events: &[Event]) -> State {
    for &event in events {
        state = apply_event(state, event);
    }
    state
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, State};

    #[test]
    fn resolved_item_can_be_closed() {
        let end = apply_events(State::Open, &[Event::Start, Event::Resolve, Event::Close]);
        assert_eq!(end, State::Closed);
    }

    #[test]
    fn reopen_from_resolved_goes_back_to_in_progress() {
        let end = apply_events(State::Open, &[Event::Start, Event::Resolve, Event::Reopen]);
        assert_eq!(end, State::InProgress);
    }

    #[test]
    fn reopened_closed_item_can_be_resolved_again() {
        let end = apply_events(
            State::Open,
            &[
                Event::Start,
                Event::Resolve,
                Event::Close,
                Event::Reopen,
                Event::Resolve,
            ],
        );
        assert_eq!(end, State::Resolved);
    }
}
