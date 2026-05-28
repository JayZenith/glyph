#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
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
    Reopen,
    Close,
}

pub fn apply_event(status: Status, event: Event) -> Status {
    match (status, event) {
        (Status::Open, Event::Start) => Status::InProgress,
        (Status::InProgress, Event::Block) => Status::Blocked,
        (Status::Blocked, Event::Unblock) => Status::InProgress,
        (Status::InProgress, Event::Resolve) => Status::Resolved,
        (Status::Blocked, Event::Resolve) => Status::Resolved,
        (Status::Resolved, Event::Close) => Status::Closed,
        (Status::Resolved, Event::Reopen) => Status::Open,
        (Status::Closed, Event::Reopen) => Status::Open,
        _ => status,
    }
}

pub fn fold_events(initial: Status, events: &[Event]) -> Status {
    events.iter().copied().fold(initial, apply_event)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_progression_reaches_closed() {
        let events = [Event::Start, Event::Resolve, Event::Close];
        assert_eq!(fold_events(Status::Open, &events), Status::Closed);
    }

    #[test]
    fn blocked_ticket_returns_to_in_progress_when_unblocked() {
        let events = [Event::Start, Event::Block, Event::Unblock];
        assert_eq!(fold_events(Status::Open, &events), Status::InProgress);
    }

    #[test]
    fn reopening_resolved_ticket_goes_back_to_in_progress() {
        let events = [Event::Start, Event::Resolve, Event::Reopen];
        assert_eq!(fold_events(Status::Open, &events), Status::InProgress);
    }

    #[test]
    fn closing_is_allowed_from_resolved_or_blocked_only() {
        assert_eq!(apply_event(Status::Resolved, Event::Close), Status::Closed);
        assert_eq!(apply_event(Status::Blocked, Event::Close), Status::Closed);
        assert_eq!(apply_event(Status::InProgress, Event::Close), Status::InProgress);
        assert_eq!(apply_event(Status::Open, Event::Close), Status::Open);
    }

    #[test]
    fn reopening_closed_ticket_requires_work_again() {
        assert_eq!(apply_event(Status::Closed, Event::Reopen), Status::InProgress);
    }
}
