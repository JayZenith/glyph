#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Open,
    InProgress,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
    Close,
    Reopen,
}

pub fn apply_events(events: &[Event]) -> Status {
    let mut status = Status::Open;

    for event in events {
        status = match (status, event) {
            (Status::Open, Event::Start) => Status::InProgress,
            (Status::Open, Event::Close) => Status::Closed,
            (Status::Open, Event::Reopen) => Status::Open,
            (Status::InProgress, Event::Start) => Status::InProgress,
            (Status::InProgress, Event::Close) => Status::Closed,
            (Status::InProgress, Event::Reopen) => Status::Open,
            (Status::Closed, Event::Start) => Status::InProgress,
            (Status::Closed, Event::Close) => Status::Closed,
            (Status::Closed, Event::Reopen) => Status::Open,
        };
    }

    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn close_from_open_is_allowed() {
        assert_eq!(apply_events(&[Event::Close]), Status::Closed);
    }

    #[test]
    fn reopening_closed_ticket_goes_to_open() {
        assert_eq!(apply_events(&[Event::Close, Event::Reopen]), Status::Open);
    }

    #[test]
    fn reopen_only_resets_after_closed() {
        assert_eq!(apply_events(&[Event::Start, Event::Reopen]), Status::InProgress);
    }

    #[test]
    fn start_does_not_revive_closed_ticket() {
        assert_eq!(apply_events(&[Event::Close, Event::Start]), Status::Closed);
    }
}
