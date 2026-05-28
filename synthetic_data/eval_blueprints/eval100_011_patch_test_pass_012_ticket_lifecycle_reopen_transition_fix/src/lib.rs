#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    New,
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

pub fn apply_events(events: &[Event]) -> Status {
    let mut status = Status::New;

    for event in events {
        status = match (status, event) {
            (Status::New, Event::Start) => Status::InProgress,
            (Status::InProgress, Event::Block) => Status::Blocked,
            (Status::Blocked, Event::Unblock) => Status::InProgress,
            (Status::InProgress, Event::Resolve) => Status::Resolved,
            (Status::Resolved, Event::Close) => Status::Closed,
            (Status::Blocked, Event::Resolve) => Status::Resolved,
            (Status::Resolved, Event::Reopen) => Status::New,
            (Status::Closed, Event::Reopen) => Status::New,
            _ => status,
        };
    }

    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn basic_happy_path_closes_ticket() {
        let events = [Event::Start, Event::Resolve, Event::Close];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn blocked_ticket_can_be_resolved_directly() {
        let events = [Event::Start, Event::Block, Event::Resolve];
        assert_eq!(apply_events(&events), Status::Resolved);
    }

    #[test]
    fn reopen_from_resolved_goes_back_to_in_progress() {
        let events = [Event::Start, Event::Resolve, Event::Reopen];
        assert_eq!(apply_events(&events), Status::InProgress);
    }

    #[test]
    fn reopen_from_closed_then_resolve_again() {
        let events = [
            Event::Start,
            Event::Resolve,
            Event::Close,
            Event::Reopen,
            Event::Resolve,
        ];
        assert_eq!(apply_events(&events), Status::Resolved);
    }

    #[test]
    fn invalid_events_do_not_change_state() {
        let events = [Event::Close, Event::Reopen, Event::Block, Event::Unblock];
        assert_eq!(apply_events(&events), Status::New);
    }
}
