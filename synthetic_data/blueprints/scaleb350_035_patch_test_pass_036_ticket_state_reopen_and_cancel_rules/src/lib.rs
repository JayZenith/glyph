#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    New,
    InProgress,
    Blocked,
    Resolved,
    Closed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
    Block,
    Unblock,
    Resolve,
    Close,
    Reopen,
    Cancel,
}

pub fn apply_events(events: &[Event]) -> Status {
    let mut status = Status::New;

    for event in events {
        status = match (status, event) {
            (Status::New, Event::Start) => Status::InProgress,
            (Status::InProgress, Event::Block) => Status::Blocked,
            (Status::Blocked, Event::Unblock) => Status::InProgress,
            (Status::InProgress, Event::Resolve) | (Status::Blocked, Event::Resolve) => Status::Resolved,
            (Status::Resolved, Event::Close) => Status::Closed,
            (_, Event::Cancel) => Status::Cancelled,
            (Status::Resolved, Event::Reopen) | (Status::Closed, Event::Reopen) => Status::New,
            _ => status,
        };
    }

    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn basic_progression_reaches_closed() {
        let events = [Event::Start, Event::Resolve, Event::Close];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn unblock_then_resolve_works() {
        let events = [Event::Start, Event::Block, Event::Unblock, Event::Resolve];
        assert_eq!(apply_events(&events), Status::Resolved);
    }

    #[test]
    fn reopen_from_resolved_returns_to_in_progress() {
        let events = [Event::Start, Event::Resolve, Event::Reopen];
        assert_eq!(apply_events(&events), Status::InProgress);
    }

    #[test]
    fn reopen_from_closed_returns_to_in_progress_and_can_finish_again() {
        let events = [
            Event::Start,
            Event::Resolve,
            Event::Close,
            Event::Reopen,
            Event::Resolve,
            Event::Close,
        ];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn cancel_is_terminal_even_if_more_events_follow() {
        let events = [
            Event::Start,
            Event::Resolve,
            Event::Cancel,
            Event::Reopen,
            Event::Close,
            Event::Start,
        ];
        assert_eq!(apply_events(&events), Status::Cancelled);
    }

    #[test]
    fn cancel_from_new_is_allowed() {
        let events = [Event::Cancel];
        assert_eq!(apply_events(&events), Status::Cancelled);
    }

    #[test]
    fn invalid_events_do_not_change_state() {
        let events = [Event::Close, Event::Reopen, Event::Start];
        assert_eq!(apply_events(&events), Status::InProgress);
    }
}
