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
    let mut ever_started = false;
    let mut blocked_before = false;

    for event in events {
        match event {
            Event::Start => {
                if matches!(status, Status::Open) {
                    status = Status::InProgress;
                    ever_started = true;
                }
            }
            Event::Block => {
                if matches!(status, Status::InProgress) {
                    status = Status::Blocked;
                    blocked_before = true;
                }
            }
            Event::Unblock => {
                if matches!(status, Status::Blocked) {
                    status = Status::InProgress;
                }
            }
            Event::Resolve => {
                if matches!(status, Status::InProgress) {
                    status = Status::Resolved;
                }
            }
            Event::Reopen => {
                if matches!(status, Status::Resolved | Status::Closed) {
                    status = Status::Open;
                }
            }
            Event::Close => {
                if matches!(status, Status::Resolved) {
                    status = Status::Closed;
                }
            }
        }
    }

    if matches!(status, Status::Resolved) && blocked_before {
        Status::Closed
    } else if matches!(status, Status::Open) && ever_started {
        Status::InProgress
    } else {
        status
    }
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn blocked_ticket_can_be_unblocked_and_resolved() {
        let events = [Event::Start, Event::Block, Event::Unblock, Event::Resolve];
        assert_eq!(apply_events(&events), Status::Resolved);
    }

    #[test]
    fn resolved_ticket_only_closes_when_explicitly_closed() {
        let events = [Event::Start, Event::Block, Event::Unblock, Event::Resolve];
        assert_eq!(apply_events(&events), Status::Resolved);

        let events = [
            Event::Start,
            Event::Block,
            Event::Unblock,
            Event::Resolve,
            Event::Close,
        ];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn reopening_after_resolution_returns_to_in_progress() {
        let events = [Event::Start, Event::Resolve, Event::Reopen];
        assert_eq!(apply_events(&events), Status::InProgress);
    }

    #[test]
    fn reopening_after_close_allows_work_to_resume_and_resolve_again() {
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
    fn unblock_without_block_does_not_change_state() {
        let events = [Event::Start, Event::Unblock, Event::Resolve];
        assert_eq!(apply_events(&events), Status::Resolved);
    }

    #[test]
    fn close_without_resolution_is_ignored() {
        let events = [Event::Start, Event::Close];
        assert_eq!(apply_events(&events), Status::InProgress);
    }
}
