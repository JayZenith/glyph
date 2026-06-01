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

pub fn apply_event(status: Status, event: Event) -> Status {
    use Event::*;
    use Status::*;

    match (status, event) {
        (Open, Start) => InProgress,
        (Open, Resolve) => Resolved,
        (Open, Close) => Closed,
        (Open, Reopen) => Open,

        (InProgress, Start) => InProgress,
        (InProgress, Resolve) => Resolved,
        (InProgress, Close) => Closed,
        (InProgress, Reopen) => InProgress,

        (Resolved, Start) => InProgress,
        (Resolved, Resolve) => Resolved,
        (Resolved, Close) => Closed,
        (Resolved, Reopen) => InProgress,

        (Closed, Start) => Closed,
        (Closed, Resolve) => Closed,
        (Closed, Close) => Closed,
        (Closed, Reopen) => Open,
    }
}

pub fn apply_events(initial: Status, events: &[Event]) -> Status {
    events.iter().copied().fold(initial, apply_event)
}

#[cfg(test)]
mod tests {
    use super::{apply_event, apply_events, Event::*, Status::*};

    #[test]
    fn reopen_from_closed_goes_to_open_then_can_restart() {
        let status = apply_events(Closed, &[Reopen, Start]);
        assert_eq!(status, InProgress);
    }

    #[test]
    fn reopen_from_resolved_returns_to_in_progress() {
        assert_eq!(apply_event(Resolved, Reopen), InProgress);
    }

    #[test]
    fn close_is_terminal_until_reopen() {
        let status = apply_events(Open, &[Start, Close, Resolve, Start]);
        assert_eq!(status, Closed);
    }

    #[test]
    fn resolve_from_open_is_allowed() {
        assert_eq!(apply_event(Open, Resolve), Resolved);
    }

    #[test]
    fn mixed_sequence_preserves_last_valid_state_transition() {
        let status = apply_events(Open, &[Start, Resolve, Reopen, Resolve, Close, Reopen]);
        assert_eq!(status, Open);
    }
}
