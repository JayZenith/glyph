#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
    Resolve,
    Close,
    Reopen,
}

pub fn apply_events(events: &[Event]) -> State {
    let mut state = State::Open;

    for event in events {
        state = match (state, event) {
            (State::Open, Event::Start) => State::InProgress,
            (State::Open, Event::Close) => State::Closed,
            (State::InProgress, Event::Resolve) => State::Resolved,
            (State::InProgress, Event::Close) => State::Closed,
            (State::Resolved, Event::Close) => State::Closed,
            (State::Resolved, Event::Reopen) => State::Open,
            (State::Closed, Event::Reopen) => State::Open,
            (s, _) => s,
        };
    }

    state
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, State};

    #[test]
    fn close_requires_resolution() {
        let state = apply_events(&[Event::Start, Event::Close]);
        assert_eq!(state, State::InProgress);
    }

    #[test]
    fn reopen_from_resolved_goes_back_to_in_progress() {
        let state = apply_events(&[Event::Start, Event::Resolve, Event::Reopen]);
        assert_eq!(state, State::InProgress);
    }

    #[test]
    fn reopen_from_closed_returns_to_open() {
        let state = apply_events(&[Event::Close, Event::Reopen]);
        assert_eq!(state, State::Open);
    }

    #[test]
    fn resolved_then_closed_stays_closed() {
        let state = apply_events(&[Event::Start, Event::Resolve, Event::Close]);
        assert_eq!(state, State::Closed);
    }
}
