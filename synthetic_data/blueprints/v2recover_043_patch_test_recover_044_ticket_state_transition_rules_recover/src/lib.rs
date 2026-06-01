#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Open,
    InProgress,
    Blocked,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
    Block,
    Unblock,
    Resolve,
    Reopen,
}

pub fn advance(status: Status, event: Event) -> Status {
    match (status, event) {
        (Status::Open, Event::Start) => Status::InProgress,
        (Status::InProgress, Event::Block) => Status::Blocked,
        (Status::Blocked, Event::Unblock) => Status::Open,
        (Status::InProgress, Event::Resolve) => Status::Closed,
        (Status::Closed, Event::Reopen) => Status::InProgress,
        _ => status,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_flow_reaches_closed() {
        let mut s = Status::Open;
        s = advance(s, Event::Start);
        s = advance(s, Event::Resolve);
        assert_eq!(s, Status::Closed);
    }

    #[test]
    fn blocked_ticket_resumes_work_when_unblocked() {
        let mut s = Status::Open;
        s = advance(s, Event::Start);
        s = advance(s, Event::Block);
        s = advance(s, Event::Unblock);
        assert_eq!(s, Status::InProgress);
    }

    #[test]
    fn closed_ticket_reopens_to_open_state() {
        let s = advance(Status::Closed, Event::Reopen);
        assert_eq!(s, Status::Open);
    }

    #[test]
    fn irrelevant_events_leave_state_unchanged() {
        assert_eq!(advance(Status::Open, Event::Resolve), Status::Open);
        assert_eq!(advance(Status::Closed, Event::Block), Status::Closed);
    }
}
