#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
    Resolve,
    Close,
    Reopen,
}

pub fn apply_events(events: &[Event]) -> Status {
    let mut status = Status::Open;

    for event in events {
        status = match (status, event) {
            (Status::Open, Event::Start) => Status::InProgress,
            (Status::Open, Event::Resolve) => Status::Resolved,
            (Status::InProgress, Event::Resolve) => Status::Resolved,
            (Status::Resolved, Event::Close) => Status::Closed,
            (Status::Resolved, Event::Reopen) => Status::Open,
            (Status::Closed, Event::Reopen) => Status::Open,
            _ => status,
        };
    }

    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn ignores_invalid_close_before_resolution() {
        let events = [Event::Close, Event::Start];
        assert_eq!(apply_events(&events), Status::InProgress);
    }

    #[test]
    fn reopening_resolved_ticket_goes_back_to_in_progress() {
        let events = [Event::Start, Event::Resolve, Event::Reopen];
        assert_eq!(apply_events(&events), Status::InProgress);
    }

    #[test]
    fn reopening_closed_ticket_then_resolving_skips_restart() {
        let events = [Event::Start, Event::Resolve, Event::Close, Event::Reopen, Event::Resolve];
        assert_eq!(apply_events(&events), Status::Resolved);
    }

    #[test]
    fn open_ticket_cannot_resolve_without_starting() {
        let events = [Event::Resolve, Event::Close];
        assert_eq!(apply_events(&events), Status::Open);
    }
}
