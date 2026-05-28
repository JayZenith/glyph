#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    New,
    Active,
    Waiting,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    StartWork,
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
            (Status::New, Event::StartWork) => Status::Active,
            (Status::New, Event::Block) => Status::Waiting,
            (Status::Active, Event::Block) => Status::Waiting,
            (Status::Waiting, Event::Unblock) => Status::Active,
            (_, Event::Resolve) => Status::Resolved,
            (_, Event::Close) => Status::Closed,
            (_, Event::Reopen) => Status::Active,
            _ => status,
        };
    }

    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn normal_flow_reaches_closed() {
        let events = [Event::StartWork, Event::Resolve, Event::Close];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn resolve_requires_started_work() {
        let events = [Event::Resolve];
        assert_eq!(apply_events(&events), Status::New);
    }

    #[test]
    fn waiting_ticket_can_be_resolved_without_unblock() {
        let events = [Event::StartWork, Event::Block, Event::Resolve];
        assert_eq!(apply_events(&events), Status::Resolved);
    }

    #[test]
    fn closed_ticket_stays_closed_on_non_reopen_events() {
        let events = [Event::StartWork, Event::Resolve, Event::Close, Event::Block, Event::Resolve];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn reopen_only_works_from_resolved_or_closed() {
        let events = [Event::Reopen, Event::StartWork];
        assert_eq!(apply_events(&events), Status::Active);

        let events = [Event::StartWork, Event::Reopen];
        assert_eq!(apply_events(&events), Status::Active);
    }

    #[test]
    fn reopen_after_close_returns_to_active() {
        let events = [Event::StartWork, Event::Resolve, Event::Close, Event::Reopen];
        assert_eq!(apply_events(&events), Status::Active);
    }

    #[test]
    fn close_requires_resolution_first() {
        let events = [Event::StartWork, Event::Close];
        assert_eq!(apply_events(&events), Status::Active);
    }
}
