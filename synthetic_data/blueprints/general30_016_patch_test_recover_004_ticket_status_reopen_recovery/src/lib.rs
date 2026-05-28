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
        (Status::Resolved, Event::Reopen) => Status::Open,
        (Status::Closed, Event::Reopen) => Status::Open,
        _ => status,
    }
}

#[cfg(test)]
mod tests {
    use super::{apply_event, Event, Status};

    #[test]
    fn progresses_through_normal_flow() {
        let mut s = Status::Open;
        s = apply_event(s, Event::Start);
        s = apply_event(s, Event::Resolve);
        s = apply_event(s, Event::Close);
        assert_eq!(s, Status::Closed);
    }

    #[test]
    fn reopen_from_resolved_returns_to_in_progress() {
        assert_eq!(apply_event(Status::Resolved, Event::Reopen), Status::InProgress);
    }

    #[test]
    fn reopen_from_closed_returns_to_in_progress() {
        assert_eq!(apply_event(Status::Closed, Event::Reopen), Status::InProgress);
    }

    #[test]
    fn invalid_events_leave_status_unchanged() {
        assert_eq!(apply_event(Status::Open, Event::Close), Status::Open);
        assert_eq!(apply_event(Status::Closed, Event::Resolve), Status::Closed);
    }
}
