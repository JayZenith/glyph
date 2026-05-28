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
        status = match (*event, status) {
            (Event::Start, Status::Open) => Status::InProgress,
            (Event::Close, _) => Status::Closed,
            (Event::Reopen, _) => Status::Open,
            _ => status,
        };
    }

    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn start_then_close_ends_closed() {
        let events = [Event::Start, Event::Close];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn reopen_after_close_returns_to_open() {
        let events = [Event::Close, Event::Reopen];
        assert_eq!(apply_events(&events), Status::Open);
    }

    #[test]
    fn reopen_without_being_closed_changes_nothing() {
        let events = [Event::Reopen];
        assert_eq!(apply_events(&events), Status::Open);

        let events = [Event::Start, Event::Reopen];
        assert_eq!(apply_events(&events), Status::InProgress);
    }

    #[test]
    fn close_from_open_is_ignored() {
        let events = [Event::Close];
        assert_eq!(apply_events(&events), Status::Open);
    }
}
