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

pub fn apply_event(status: Status, event: Event) -> Status {
    match (status, event) {
        (Status::Open, Event::Start) => Status::InProgress,
        (Status::Open, Event::Resolve) => Status::Resolved,
        (Status::InProgress, Event::Resolve) => Status::Resolved,
        (Status::Resolved, Event::Close) => Status::Closed,
        (Status::Resolved, Event::Reopen) => Status::Open,
        (Status::Closed, Event::Reopen) => Status::Open,
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
    use super::{apply_event, apply_events, Event, Status};

    #[test]
    fn reopen_from_resolved_goes_back_to_in_progress() {
        assert_eq!(apply_event(Status::Resolved, Event::Reopen), Status::InProgress);
    }

    #[test]
    fn closed_ticket_stays_closed_on_reopen() {
        assert_eq!(apply_event(Status::Closed, Event::Reopen), Status::Closed);
    }

    #[test]
    fn lifecycle_reopen_then_resolve_again() {
        let events = [Event::Start, Event::Resolve, Event::Reopen, Event::Resolve, Event::Close];
        assert_eq!(apply_events(Status::Open, &events), Status::Closed);
    }
}
