#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TicketState {
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

pub fn apply_event(state: TicketState, event: Event) -> TicketState {
    match (state, event) {
        (TicketState::Open, Event::Start) => TicketState::InProgress,
        (TicketState::InProgress, Event::Block) => TicketState::Blocked,
        (TicketState::Blocked, Event::Unblock) => TicketState::Open,
        (TicketState::InProgress, Event::Resolve) => TicketState::Resolved,
        (TicketState::Resolved, Event::Close) => TicketState::Closed,
        (TicketState::Resolved, Event::Reopen) => TicketState::Resolved,
        (TicketState::Closed, Event::Reopen) => TicketState::Open,
        _ => state,
    }
}

pub fn apply_events(mut state: TicketState, events: &[Event]) -> TicketState {
    for &event in events {
        state = apply_event(state, event);
    }
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progresses_and_closes() {
        let events = [Event::Start, Event::Resolve, Event::Close];
        assert_eq!(apply_events(TicketState::Open, &events), TicketState::Closed);
    }

    #[test]
    fn unblock_returns_to_in_progress() {
        let events = [Event::Start, Event::Block, Event::Unblock];
        assert_eq!(apply_events(TicketState::Open, &events), TicketState::InProgress);
    }

    #[test]
    fn reopening_resolved_work_goes_back_to_in_progress() {
        let events = [Event::Start, Event::Resolve, Event::Reopen];
        assert_eq!(apply_events(TicketState::Open, &events), TicketState::InProgress);
    }

    #[test]
    fn close_without_resolve_is_ignored() {
        let events = [Event::Close, Event::Start];
        assert_eq!(apply_events(TicketState::Open, &events), TicketState::InProgress);
    }
}
