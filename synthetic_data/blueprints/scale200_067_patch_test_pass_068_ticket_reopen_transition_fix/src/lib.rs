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
        (Status::Closed, Event::Reopen) => Status::InProgress,
        (Status::Resolved, Event::Reopen) => Status::Resolved,
        _ => status,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_progression_reaches_closed() {
        let mut s = Status::Open;
        s = apply_event(s, Event::Start);
        s = apply_event(s, Event::Resolve);
        s = apply_event(s, Event::Close);
        assert_eq!(s, Status::Closed);
    }

    #[test]
    fn reopen_from_closed_returns_to_in_progress() {
        assert_eq!(apply_event(Status::Closed, Event::Reopen), Status::InProgress);
    }

    #[test]
    fn reopen_from_resolved_returns_to_in_progress() {
        assert_eq!(apply_event(Status::Resolved, Event::Reopen), Status::InProgress);
    }

    #[test]
    fn invalid_transition_keeps_state() {
        assert_eq!(apply_event(Status::Open, Event::Close), Status::Open);
    }
}
