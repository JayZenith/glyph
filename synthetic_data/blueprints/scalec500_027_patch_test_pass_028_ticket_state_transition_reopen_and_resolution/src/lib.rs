#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    New,
    InProgress,
    Waiting,
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
            (Status::InProgress, Event::Block) => Status::Waiting,
            (Status::Waiting, Event::Unblock) => Status::InProgress,
            (Status::InProgress, Event::Resolve) => Status::Resolved,
            (Status::Resolved, Event::Close) => Status::Closed,
            (Status::Resolved, Event::Reopen) => Status::New,
            (Status::Closed, Event::Reopen) => Status::New,
            (s, _) => s,
        };
    }
    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn basic_progression_to_closed() {
        let events = [Event::Start, Event::Resolve, Event::Close];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn blocked_work_can_resume_and_resolve() {
        let events = [Event::Start, Event::Block, Event::Unblock, Event::Resolve];
        assert_eq!(apply_events(&events), Status::Resolved);
    }

    #[test]
    fn reopen_from_resolved_returns_to_in_progress() {
        let events = [Event::Start, Event::Resolve, Event::Reopen];
        assert_eq!(apply_events(&events), Status::InProgress);
    }

    #[test]
    fn reopen_from_closed_then_start_should_not_reset_twice() {
        let events = [Event::Start, Event::Resolve, Event::Close, Event::Reopen, Event::Start];
        assert_eq!(apply_events(&events), Status::InProgress);
    }

    #[test]
    fn resolve_after_waiting_without_unblock_is_ignored() {
        let events = [Event::Start, Event::Block, Event::Resolve];
        assert_eq!(apply_events(&events), Status::Waiting);
    }

    #[test]
    fn reopened_work_can_be_blocked_again_and_closed() {
        let events = [
            Event::Start,
            Event::Resolve,
            Event::Reopen,
            Event::Block,
            Event::Unblock,
            Event::Resolve,
            Event::Close,
        ];
        assert_eq!(apply_events(&events), Status::Closed);
    }
}
