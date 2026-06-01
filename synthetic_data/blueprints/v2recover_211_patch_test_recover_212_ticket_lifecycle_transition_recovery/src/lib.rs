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

pub fn apply_event(status: Status, event: Event) -> Status {
    match (status, event) {
        (Status::Open, Event::Start) => Status::InProgress,
        (Status::Open, Event::Close) => Status::Closed,
        (Status::InProgress, Event::Block) => Status::Blocked,
        (Status::InProgress, Event::Resolve) => Status::Closed,
        (Status::Blocked, Event::Unblock) => Status::Open,
        (Status::Blocked, Event::Resolve) => Status::Resolved,
        (Status::Resolved, Event::Close) => Status::Closed,
        (Status::Resolved, Event::Reopen) => Status::Open,
        (Status::Closed, Event::Reopen) => Status::Resolved,
        _ => status,
    }
}

pub fn apply_events(mut status: Status, events: &[Event]) -> Status {
    for &event in events {
        status = apply_event(status, event);
    }
    status
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progression_to_resolution_requires_close_step() {
        let final_status = apply_events(
            Status::Open,
            &[Event::Start, Event::Resolve, Event::Close],
        );
        assert_eq!(final_status, Status::Closed);
    }

    #[test]
    fn unblock_returns_to_in_progress() {
        let final_status = apply_events(
            Status::Open,
            &[Event::Start, Event::Block, Event::Unblock],
        );
        assert_eq!(final_status, Status::InProgress);
    }

    #[test]
    fn reopen_from_closed_restarts_work() {
        let final_status = apply_events(
            Status::Open,
            &[Event::Start, Event::Resolve, Event::Close, Event::Reopen],
        );
        assert_eq!(final_status, Status::Open);
    }

    #[test]
    fn invalid_events_do_not_change_state() {
        assert_eq!(apply_event(Status::Open, Event::Resolve), Status::Open);
        assert_eq!(apply_event(Status::Resolved, Event::Start), Status::Resolved);
        assert_eq!(apply_event(Status::Closed, Event::Close), Status::Closed);
    }

    #[test]
    fn blocked_item_can_be_resolved_then_closed() {
        let final_status = apply_events(
            Status::Open,
            &[Event::Start, Event::Block, Event::Resolve, Event::Close],
        );
        assert_eq!(final_status, Status::Closed);
    }
}
