#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Open,
    InProgress,
    Blocked,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Start,
    Block,
    Unblock,
    Resolve,
    Reopen,
    Close,
}

pub fn apply_events(events: &[Event]) -> Status {
    let mut status = Status::Open;

    for event in events {
        status = match (&status, event) {
            (Status::Open, Event::Start) => Status::InProgress,
            (Status::InProgress, Event::Block) => Status::Blocked,
            (Status::Blocked, Event::Unblock) => Status::Open,
            (_, Event::Resolve) => Status::Resolved,
            (_, Event::Reopen) => Status::Open,
            (_, Event::Close) => Status::Closed,
            _ => status,
        };
    }

    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn reopen_then_start_and_resolve_can_close() {
        let events = [Event::Resolve, Event::Reopen, Event::Start, Event::Resolve, Event::Close];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn unblock_returns_to_in_progress() {
        let events = [Event::Start, Event::Block, Event::Unblock];
        assert_eq!(apply_events(&events), Status::InProgress);
    }

    #[test]
    fn cannot_resolve_without_work_starting() {
        let events = [Event::Resolve];
        assert_eq!(apply_events(&events), Status::Open);
    }

    #[test]
    fn close_requires_resolved_state() {
        let events = [Event::Start, Event::Close];
        assert_eq!(apply_events(&events), Status::InProgress);
    }

    #[test]
    fn reopen_from_closed_goes_to_in_progress() {
        let events = [Event::Start, Event::Resolve, Event::Close, Event::Reopen];
        assert_eq!(apply_events(&events), Status::InProgress);
    }
}
