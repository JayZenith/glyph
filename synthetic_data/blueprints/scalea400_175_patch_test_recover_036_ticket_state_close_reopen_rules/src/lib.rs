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
        (_, Event::Close) => Status::Closed,
        (_, Event::Reopen) => Status::Open,
        _ => status,
    }
}

#[cfg(test)]
mod tests {
    use super::{apply_event, Event, Status};

    #[test]
    fn normal_progression_reaches_closed() {
        let s = apply_event(Status::Open, Event::Start);
        let s = apply_event(s, Event::Resolve);
        let s = apply_event(s, Event::Close);
        assert_eq!(s, Status::Closed);
    }

    #[test]
    fn only_resolved_can_be_closed() {
        assert_eq!(apply_event(Status::Open, Event::Close), Status::Open);
        assert_eq!(apply_event(Status::InProgress, Event::Close), Status::InProgress);
        assert_eq!(apply_event(Status::Resolved, Event::Close), Status::Closed);
    }

    #[test]
    fn only_closed_tickets_can_reopen() {
        assert_eq!(apply_event(Status::Open, Event::Reopen), Status::Open);
        assert_eq!(apply_event(Status::Resolved, Event::Reopen), Status::Resolved);
        assert_eq!(apply_event(Status::Closed, Event::Reopen), Status::Open);
    }
}
