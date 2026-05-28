#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Open,
    InProgress,
    Blocked,
    Resolved,
    Closed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    StartWork,
    Block,
    Unblock,
    Resolve,
    Close,
    Reopen,
    Cancel,
}

pub fn apply_events(events: &[Event]) -> Status {
    let mut status = Status::Open;
    for &event in events {
        status = match (status, event) {
            (Status::Open, Event::StartWork) => Status::InProgress,
            (Status::Open, Event::Cancel) => Status::Cancelled,
            (Status::InProgress, Event::Block) => Status::Blocked,
            (Status::InProgress, Event::Resolve) => Status::Resolved,
            (Status::InProgress, Event::Cancel) => Status::Cancelled,
            (Status::Blocked, Event::Unblock) => Status::Open,
            (Status::Blocked, Event::Resolve) => Status::Resolved,
            (Status::Blocked, Event::Cancel) => Status::Cancelled,
            (Status::Resolved, Event::Close) => Status::Closed,
            (Status::Resolved, Event::Reopen) => Status::Open,
            (Status::Closed, Event::Reopen) => Status::Open,
            (s, _) => s,
        };
    }
    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event::*, Status};

    #[test]
    fn blocked_unblock_returns_to_active_work() {
        let events = [StartWork, Block, Unblock];
        assert_eq!(apply_events(&events), Status::InProgress);
    }

    #[test]
    fn reopen_from_resolved_requires_closing_again() {
        let events = [StartWork, Resolve, Reopen, Resolve, Close];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn cancelled_is_terminal() {
        let events = [StartWork, Cancel, Reopen, StartWork, Resolve, Close];
        assert_eq!(apply_events(&events), Status::Cancelled);
    }

    #[test]
    fn closed_is_terminal_for_non_reopen_events() {
        let events = [StartWork, Resolve, Close, Resolve, Cancel, Block];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn invalid_events_do_not_change_state() {
        let events = [Resolve, Close, StartWork, Close, Resolve, Close];
        assert_eq!(apply_events(&events), Status::Closed);
    }
}
