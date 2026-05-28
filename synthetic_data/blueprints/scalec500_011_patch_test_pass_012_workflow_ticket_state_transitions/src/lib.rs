#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Open,
    InProgress,
    Blocked,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
    Block,
    Unblock,
    Resolve,
    Close,
    Reopen,
}

pub fn apply_events(events: &[Event]) -> State {
    let mut state = State::Open;
    let mut last_active = State::Open;

    for event in events {
        match (*event, state) {
            (Event::Start, State::Open) => {
                state = State::InProgress;
                last_active = state;
            }
            (Event::Block, State::InProgress) => {
                state = State::Blocked;
            }
            (Event::Unblock, State::Blocked) => {
                state = State::Open;
            }
            (Event::Resolve, State::InProgress) | (Event::Resolve, State::Blocked) => {
                state = State::Resolved;
            }
            (Event::Close, State::Resolved) => {
                state = State::Closed;
            }
            (Event::Reopen, State::Resolved) | (Event::Reopen, State::Closed) => {
                state = State::Open;
                last_active = state;
            }
            _ => {}
        }
    }

    state
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, State};

    #[test]
    fn unblock_returns_to_last_active_work_state() {
        let events = [Event::Start, Event::Block, Event::Unblock];
        assert_eq!(apply_events(&events), State::InProgress);
    }

    #[test]
    fn blocked_ticket_cannot_resolve_directly() {
        let events = [Event::Start, Event::Block, Event::Resolve, Event::Unblock];
        assert_eq!(apply_events(&events), State::InProgress);
    }

    #[test]
    fn reopen_from_closed_goes_back_to_open() {
        let events = [Event::Start, Event::Resolve, Event::Close, Event::Reopen];
        assert_eq!(apply_events(&events), State::Open);
    }

    #[test]
    fn close_requires_resolved_state() {
        let events = [Event::Start, Event::Close, Event::Resolve];
        assert_eq!(apply_events(&events), State::Resolved);
    }

    #[test]
    fn reopening_resolved_then_starting_allows_closure_again() {
        let events = [
            Event::Start,
            Event::Resolve,
            Event::Reopen,
            Event::Start,
            Event::Resolve,
            Event::Close,
        ];
        assert_eq!(apply_events(&events), State::Closed);
    }
}
