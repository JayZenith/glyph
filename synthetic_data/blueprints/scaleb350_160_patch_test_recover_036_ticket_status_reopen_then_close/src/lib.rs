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
        status = match (*event, status) {
            (Event::Start, Status::Open) => Status::InProgress,
            (Event::Resolve, Status::InProgress) => Status::Closed,
            (Event::Reopen, Status::Closed) => Status::Open,
            (Event::Start, Status::Closed) => Status::InProgress,
            _ => status,
        };
    }

    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn resolve_closes_after_start() {
        let events = [Event::Start, Event::Resolve];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn reopen_closed_ticket_returns_to_in_progress() {
        let events = [Event::Start, Event::Resolve, Event::Reopen];
        assert_eq!(apply_events(&events), Status::InProgress);
    }

    #[test]
    fn start_does_not_revive_closed_ticket_without_reopen() {
        let events = [Event::Start, Event::Resolve, Event::Start];
        assert_eq!(apply_events(&events), Status::Closed);
    }
}
