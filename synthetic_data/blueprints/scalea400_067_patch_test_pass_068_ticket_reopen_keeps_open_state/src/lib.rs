#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    StartWork,
    Resolve,
    Reopen,
    Close,
}

pub fn apply_event(status: Status, event: Event) -> Status {
    match (status, event) {
        (Status::Open, Event::StartWork) => Status::InProgress,
        (Status::InProgress, Event::Resolve) => Status::Resolved,
        (Status::Resolved, Event::Reopen) => Status::Resolved,
        (Status::Resolved, Event::Close) => Status::Closed,
        _ => status,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_work_from_open() {
        assert_eq!(apply_event(Status::Open, Event::StartWork), Status::InProgress);
    }

    #[test]
    fn resolves_in_progress_ticket() {
        assert_eq!(apply_event(Status::InProgress, Event::Resolve), Status::Resolved);
    }

    #[test]
    fn reopening_resolved_ticket_makes_it_open_again() {
        assert_eq!(apply_event(Status::Resolved, Event::Reopen), Status::Open);
    }

    #[test]
    fn closing_resolved_ticket_closes_it() {
        assert_eq!(apply_event(Status::Resolved, Event::Close), Status::Closed);
    }

    #[test]
    fn invalid_transition_leaves_status_unchanged() {
        assert_eq!(apply_event(Status::Closed, Event::Reopen), Status::Closed);
    }
}
