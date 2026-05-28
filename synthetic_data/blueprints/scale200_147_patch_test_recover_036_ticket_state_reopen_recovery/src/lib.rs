#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Open,
    InProgress,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
    Resolve,
    Reopen,
}

pub fn apply_events(events: &[Event]) -> Status {
    let mut status = Status::Open;
    for event in events {
        status = match (status, event) {
            (Status::Open, Event::Start) => Status::InProgress,
            (Status::InProgress, Event::Resolve) => Status::Closed,
            (Status::Closed, Event::Reopen) => Status::InProgress,
            _ => status,
        };
    }
    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn resolves_after_start() {
        assert_eq!(apply_events(&[Event::Start, Event::Resolve]), Status::Closed);
    }

    #[test]
    fn reopen_then_resolve_again_closes_ticket() {
        let events = [Event::Start, Event::Resolve, Event::Reopen, Event::Resolve];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn reopen_without_restart_keeps_ticket_open() {
        let events = [Event::Start, Event::Resolve, Event::Reopen];
        assert_eq!(apply_events(&events), Status::Open);
    }
}
