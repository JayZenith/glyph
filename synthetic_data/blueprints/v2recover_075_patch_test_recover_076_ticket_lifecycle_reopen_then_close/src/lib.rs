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
        (Status::InProgress, Event::Resolve) => Status::Resolved,
        (Status::Resolved, Event::Close) => Status::Closed,
        (Status::Closed, Event::Reopen) => Status::Open,
        (s, _) => s,
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
    use super::{apply_events, Event, Status};

    #[test]
    fn happy_path_reaches_closed() {
        let events = [Event::Start, Event::Resolve, Event::Close];
        assert_eq!(apply_events(Status::Open, &events), Status::Closed);
    }

    #[test]
    fn reopen_from_closed_requires_work_again() {
        let events = [
            Event::Start,
            Event::Resolve,
            Event::Close,
            Event::Reopen,
            Event::Start,
            Event::Resolve,
            Event::Close,
        ];
        assert_eq!(apply_events(Status::Open, &events), Status::Closed);
    }

    #[test]
    fn reopen_from_resolved_goes_back_to_in_progress() {
        let events = [Event::Start, Event::Resolve, Event::Reopen];
        assert_eq!(apply_events(Status::Open, &events), Status::InProgress);
    }

    #[test]
    fn invalid_transitions_leave_status_unchanged() {
        let events = [Event::Close, Event::Resolve, Event::Reopen];
        assert_eq!(apply_events(Status::Open, &events), Status::Open);
    }
}
