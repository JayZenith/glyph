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
    use super::*;

    #[test]
    fn normal_flow_reaches_closed() {
        let mut s = Status::Open;
        s = apply_event(s, Event::Start);
        s = apply_event(s, Event::Resolve);
        s = apply_event(s, Event::Close);
        assert_eq!(s, Status::Closed);
    }

    #[test]
    fn reopen_from_resolved_goes_back_to_in_progress() {
        assert_eq!(apply_event(Status::Resolved, Event::Reopen), Status::InProgress);
    }

    #[test]
    fn closed_ticket_cannot_reopen() {
        assert_eq!(apply_event(Status::Closed, Event::Reopen), Status::Closed);
    }
}
