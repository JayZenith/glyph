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
    Cancel,
}

pub fn apply_event(status: Status, event: Event) -> Status {
    use Event::*;
    use Status::*;

    match (status, event) {
        (Open, Start) => InProgress,
        (Open, Cancel) => Closed,
        (Open, Reopen) => Open,

        (InProgress, Resolve) => Resolved,
        (InProgress, Cancel) => Closed,
        (InProgress, Reopen) => Open,
        (InProgress, Start) => InProgress,

        (Resolved, Close) => Closed,
        (Resolved, Reopen) => Open,
        (Resolved, Resolve) => Resolved,

        (Closed, Reopen) => Open,
        (Closed, Close) => Closed,

        (s, _) => s,
    }
}

pub fn apply_events(initial: Status, events: &[Event]) -> Status {
    events.iter().copied().fold(initial, apply_event)
}

#[cfg(test)]
mod tests {
    use super::{apply_event, apply_events, Event::*, Status::*};

    #[test]
    fn lifecycle_happy_path() {
        let events = [Start, Resolve, Close];
        assert_eq!(apply_events(Open, &events), Closed);
    }

    #[test]
    fn reopen_from_resolved_requires_work_again() {
        let events = [Start, Resolve, Reopen, Start, Resolve];
        assert_eq!(apply_events(Open, &events), Resolved);
    }

    #[test]
    fn close_is_ignored_until_resolved() {
        let events = [Close, Start, Close, Resolve, Close];
        assert_eq!(apply_events(Open, &events), Closed);
    }

    #[test]
    fn cancel_from_in_progress_closes_immediately() {
        let events = [Start, Cancel, Resolve, Reopen];
        assert_eq!(apply_events(Open, &events), Open);
    }

    #[test]
    fn invalid_events_leave_state_unchanged() {
        assert_eq!(apply_event(Resolved, Start), Resolved);
        assert_eq!(apply_event(Closed, Resolve), Closed);
        assert_eq!(apply_event(Open, Close), Open);
    }
}
