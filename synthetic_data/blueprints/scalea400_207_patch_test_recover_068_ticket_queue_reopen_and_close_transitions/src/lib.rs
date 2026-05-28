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
        status = match (status, *event) {
            (Status::New, Event::Start) => Status::InProgress,
            (Status::InProgress, Event::Block) => Status::Waiting,
            (Status::Waiting, Event::Unblock) => Status::InProgress,
            (Status::InProgress, Event::Resolve) | (Status::Waiting, Event::Resolve) => Status::Resolved,
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
    fn can_flow_to_closed() {
        let events = [Event::Start, Event::Resolve, Event::Close];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn waiting_can_resolve_without_unblock() {
        let events = [Event::Start, Event::Block, Event::Resolve];
        assert_eq!(apply_events(&events), Status::Resolved);
    }

    #[test]
    fn reopen_from_resolved_goes_back_to_in_progress() {
        let events = [Event::Start, Event::Resolve, Event::Reopen];
        assert_eq!(apply_events(&events), Status::InProgress);
    }

    #[test]
    fn reopen_from_closed_then_continue_work() {
        let events = [Event::Start, Event::Resolve, Event::Close, Event::Reopen, Event::Resolve, Event::Close];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn unrelated_events_do_not_change_state() {
        let events = [Event::Close, Event::Block, Event::Unblock];
        assert_eq!(apply_events(&events), Status::New);
    }
}
