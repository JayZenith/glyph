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
        (Status::Resolved, Event::Close) => Status::Resolved,
        (Status::Resolved, Event::Reopen) => Status::Resolved,
        (Status::Closed, Event::Reopen) => Status::Open,
        _ => status,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_flow_reaches_closed() {
        let mut status = Status::Open;
        status = apply_event(status, Event::Start);
        status = apply_event(status, Event::Resolve);
        status = apply_event(status, Event::Close);
        assert_eq!(status, Status::Closed);
    }

    #[test]
    fn reopen_from_resolved_goes_back_to_in_progress() {
        let status = apply_event(Status::Resolved, Event::Reopen);
        assert_eq!(status, Status::InProgress);
    }

    #[test]
    fn reopen_from_closed_returns_open() {
        let status = apply_event(Status::Closed, Event::Reopen);
        assert_eq!(status, Status::Open);
    }

    #[test]
    fn invalid_transition_keeps_state() {
        let status = apply_event(Status::Open, Event::Close);
        assert_eq!(status, Status::Open);
    }
}
