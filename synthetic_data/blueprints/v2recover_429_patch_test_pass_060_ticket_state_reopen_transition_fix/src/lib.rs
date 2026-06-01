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
            (State::Open, Event::Resolve) => State::Resolved,
            (State::InProgress, Event::Resolve) => State::Resolved,
            (State::Resolved, Event::Close) => State::Closed,
            (State::Closed, Event::Reopen) => State::Open,
            (State::Resolved, Event::Reopen) => State::Open,
            _ => state,
        };
    }

    state
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, State};

    #[test]
    fn stays_open_on_irrelevant_events() {
        assert_eq!(apply_events(&[Event::Close, Event::Reopen]), State::Open);
    }

    #[test]
    fn resolved_can_be_closed() {
        assert_eq!(apply_events(&[Event::Start, Event::Resolve, Event::Close]), State::Closed);
    }

    #[test]
    fn reopen_from_closed_requires_work_again() {
        assert_eq!(
            apply_events(&[Event::Start, Event::Resolve, Event::Close, Event::Reopen]),
            State::InProgress
        );
    }

    #[test]
    fn reopen_from_resolved_returns_to_in_progress() {
        assert_eq!(
            apply_events(&[Event::Start, Event::Resolve, Event::Reopen]),
            State::InProgress
        );
    }

    #[test]
    fn reopen_then_resolve_can_finish_again() {
        assert_eq!(
            apply_events(&[
                Event::Start,
                Event::Resolve,
                Event::Close,
                Event::Reopen,
                Event::Resolve,
                Event::Close,
            ]),
            State::Closed
        );
    }
}
