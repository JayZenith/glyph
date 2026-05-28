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

pub fn apply_events(events: &[Event]) -> Status {
    let mut status = Status::Open;

    for event in events {
        status = match (*event, status) {
            (Event::Start, Status::Open) => Status::InProgress,
            (Event::Resolve, Status::InProgress) => Status::Resolved,
            (Event::Close, Status::Resolved) => Status::Closed,
            (Event::Reopen, Status::Resolved) => Status::Open,
            (Event::Reopen, Status::Closed) => Status::Open,
            _ => status,
        };
    }

    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn normal_flow_reaches_closed() {
        let events = [Event::Start, Event::Resolve, Event::Close];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn reopen_from_resolved_goes_back_to_in_progress() {
        let events = [Event::Start, Event::Resolve, Event::Reopen];
        assert_eq!(apply_events(&events), Status::InProgress);
    }

    #[test]
    fn reopen_from_closed_allows_resolution_again() {
        let events = [
            Event::Start,
            Event::Resolve,
            Event::Close,
            Event::Reopen,
            Event::Resolve,
            Event::Close,
        ];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn invalid_close_from_in_progress_is_ignored() {
        let events = [Event::Start, Event::Close, Event::Resolve];
        assert_eq!(apply_events(&events), Status::Resolved);
    }
}
