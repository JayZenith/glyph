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

pub fn apply_event(status: Status, event: Event) -> Status {
    match (status, event) {
        (Status::Open, Event::Start) => Status::InProgress,
        (Status::InProgress, Event::Resolve) => Status::Closed,
        (Status::InProgress, Event::Reopen) => Status::Open,
        (Status::Closed, Event::Reopen) => Status::InProgress,
        _ => status,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_moves_open_to_in_progress() {
        assert_eq!(apply_event(Status::Open, Event::Start), Status::InProgress);
    }

    #[test]
    fn resolve_moves_in_progress_to_closed() {
        assert_eq!(apply_event(Status::InProgress, Event::Resolve), Status::Closed);
    }

    #[test]
    fn reopen_from_closed_goes_back_to_open() {
        assert_eq!(apply_event(Status::Closed, Event::Reopen), Status::Open);
    }

    #[test]
    fn unrelated_event_leaves_status_unchanged() {
        assert_eq!(apply_event(Status::Open, Event::Resolve), Status::Open);
    }
}
