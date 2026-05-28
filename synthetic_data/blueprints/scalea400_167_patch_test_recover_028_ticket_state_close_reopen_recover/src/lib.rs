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

pub fn apply_event(status: Status, event: Event) -> Status {
    match (status, event) {
        (Status::Open, Event::Start) => Status::InProgress,
        (_, Event::Close) => Status::Closed,
        (Status::Closed, Event::Reopen) => Status::InProgress,
        _ => status,
    }
}

#[cfg(test)]
mod tests {
    use super::{apply_event, Event, Status};

    #[test]
    fn close_only_from_in_progress() {
        assert_eq!(apply_event(Status::Open, Event::Close), Status::Open);
        assert_eq!(apply_event(Status::InProgress, Event::Close), Status::Closed);
    }

    #[test]
    fn reopen_goes_back_to_open() {
        assert_eq!(apply_event(Status::Closed, Event::Reopen), Status::Open);
    }

    #[test]
    fn unrelated_events_keep_state() {
        assert_eq!(apply_event(Status::Closed, Event::Start), Status::Closed);
        assert_eq!(apply_event(Status::Open, Event::Reopen), Status::Open);
    }
}
