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

pub fn apply_events(events: &[Event]) -> Status {
    let mut status = Status::Open;
    let mut had_resolution = false;

    for event in events {
        status = match (status, *event) {
            (Status::Open, Event::Start) => Status::InProgress,
            (Status::Open, Event::Resolve) => {
                had_resolution = true;
                Status::Resolved
            }
            (Status::InProgress, Event::Block) => Status::Blocked,
            (Status::InProgress, Event::Resolve) => {
                had_resolution = true;
                Status::Resolved
            }
            (Status::Blocked, Event::Unblock) => Status::Open,
            (Status::Blocked, Event::Resolve) => {
                had_resolution = true;
                Status::Resolved
            }
            (Status::Resolved, Event::Reopen) => Status::Open,
            (Status::Resolved, Event::Close) if had_resolution => Status::Closed,
            (Status::Closed, Event::Reopen) => Status::Open,
            (s, _) => s,
        };
    }

    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn blocked_ticket_unblocks_back_to_in_progress() {
        let events = [Event::Start, Event::Block, Event::Unblock];
        assert_eq!(apply_events(&events), Status::InProgress);
    }

    #[test]
    fn reopen_after_resolution_returns_to_in_progress() {
        let events = [Event::Start, Event::Resolve, Event::Reopen];
        assert_eq!(apply_events(&events), Status::InProgress);
    }

    #[test]
    fn closed_ticket_cannot_reopen() {
        let events = [Event::Start, Event::Resolve, Event::Close, Event::Reopen];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn duplicate_start_does_not_change_active_work() {
        let events = [Event::Start, Event::Start, Event::Block, Event::Unblock];
        assert_eq!(apply_events(&events), Status::InProgress);
    }

    #[test]
    fn unresolved_close_is_ignored() {
        let events = [Event::Start, Event::Close];
        assert_eq!(apply_events(&events), Status::InProgress);
    }

    #[test]
    fn blocked_ticket_can_resolve_and_close() {
        let events = [Event::Start, Event::Block, Event::Resolve, Event::Close];
        assert_eq!(apply_events(&events), Status::Closed);
    }
}
