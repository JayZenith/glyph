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
            (_, Event::Resolve) => Status::Closed,
            (_, Event::Reopen) => Status::Open,
            _ => status,
        };
    }

    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn resolves_only_after_work_started() {
        assert_eq!(apply_events(&[Event::Resolve]), Status::Open);
        assert_eq!(apply_events(&[Event::Start, Event::Resolve]), Status::Closed);
    }

    #[test]
    fn reopen_only_affects_closed_tickets() {
        assert_eq!(apply_events(&[Event::Reopen]), Status::Open);
        assert_eq!(apply_events(&[Event::Start, Event::Reopen]), Status::InProgress);
        assert_eq!(apply_events(&[Event::Start, Event::Resolve, Event::Reopen]), Status::Open);
    }
}
