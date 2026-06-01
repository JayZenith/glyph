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
    Close,
    Reopen,
}

pub fn apply_events(events: &[Event]) -> Status {
    let mut status = Status::Open;

    for event in events {
        status = match (status, *event) {
            (Status::Open, Event::Start) => Status::InProgress,
            (Status::InProgress, Event::Block) => Status::Blocked,
            (Status::Blocked, Event::Unblock) => Status::InProgress,
            (Status::InProgress, Event::Resolve) | (Status::Blocked, Event::Resolve) => Status::Resolved,
            (Status::Resolved, Event::Close) => Status::Closed,
            (_, Event::Reopen) => Status::Open,
            (_, _) => status,
        };
    }

    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn basic_progression_closes() {
        let events = [Event::Start, Event::Resolve, Event::Close];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn unblock_then_resolve_works() {
        let events = [Event::Start, Event::Block, Event::Unblock, Event::Resolve];
        assert_eq!(apply_events(&events), Status::Resolved);
    }

    #[test]
    fn reopen_only_from_resolved_or_closed() {
        let from_open = [Event::Reopen];
        let from_active = [Event::Start, Event::Reopen];
        let from_blocked = [Event::Start, Event::Block, Event::Reopen];
        let from_resolved = [Event::Start, Event::Resolve, Event::Reopen];
        let from_closed = [Event::Start, Event::Resolve, Event::Close, Event::Reopen];

        assert_eq!(apply_events(&from_open), Status::Open);
        assert_eq!(apply_events(&from_active), Status::InProgress);
        assert_eq!(apply_events(&from_blocked), Status::Blocked);
        assert_eq!(apply_events(&from_resolved), Status::Open);
        assert_eq!(apply_events(&from_closed), Status::Open);
    }

    #[test]
    fn closed_is_terminal_except_reopen() {
        let events = [
            Event::Start,
            Event::Resolve,
            Event::Close,
            Event::Start,
            Event::Block,
            Event::Resolve,
            Event::Close,
        ];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn invalid_events_do_not_skip_required_steps() {
        let events = [
            Event::Close,
            Event::Resolve,
            Event::Start,
            Event::Close,
            Event::Resolve,
            Event::Close,
        ];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn reopened_ticket_must_be_resolved_again_before_close() {
        let events = [
            Event::Start,
            Event::Resolve,
            Event::Close,
            Event::Reopen,
            Event::Close,
            Event::Start,
            Event::Resolve,
            Event::Close,
        ];
        assert_eq!(apply_events(&events), Status::Closed);
    }
}
