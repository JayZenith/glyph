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
            (Status::InProgress, Event::Close) => Status::Open,
            (Status::InProgress, Event::Reopen) => Status::Open,
            (Status::Closed, Event::Start) => Status::InProgress,
            (Status::Closed, Event::Close) => Status::Closed,
            (Status::Closed, Event::Reopen) => Status::Closed,
        };
    }

    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn close_from_in_progress_stays_closed() {
        let events = [Event::Start, Event::Close];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn reopen_from_closed_returns_open() {
        let events = [Event::Close, Event::Reopen];
        assert_eq!(apply_events(&events), Status::Open);
    }

    #[test]
    fn start_does_not_resume_closed_without_reopen() {
        let events = [Event::Close, Event::Start];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn regular_progression_can_restart_after_reopen() {
        let events = [Event::Start, Event::Close, Event::Reopen, Event::Start];
        assert_eq!(apply_events(&events), Status::InProgress);
    }
}
