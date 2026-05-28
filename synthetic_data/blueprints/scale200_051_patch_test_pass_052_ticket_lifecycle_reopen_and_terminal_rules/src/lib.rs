#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    New,
    InProgress,
    Blocked,
    Resolved,
    Closed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
    Block,
    Unblock,
    Resolve,
    Close,
    Reopen,
    Cancel,
}

pub fn apply_event(status: Status, event: Event) -> Status {
    use Event::*;
    use Status::*;

    match (status, event) {
        (New, Start) => InProgress,
        (New, Cancel) => Cancelled,
        (InProgress, Block) => Blocked,
        (InProgress, Resolve) => Resolved,
        (InProgress, Cancel) => Cancelled,
        (Blocked, Unblock) => InProgress,
        (Blocked, Resolve) => Resolved,
        (Blocked, Cancel) => Cancelled,
        (Resolved, Close) => Closed,
        (Resolved, Reopen) => New,
        (Closed, Reopen) => New,
        (s, _) => s,
    }
}

pub fn fold_events(initial: Status, events: &[Event]) -> Status {
    events.iter().copied().fold(initial, apply_event)
}

#[cfg(test)]
mod tests {
    use super::{apply_event, fold_events, Event::*, Status::*};

    #[test]
    fn resolved_reopens_into_in_progress() {
        assert_eq!(apply_event(Resolved, Reopen), InProgress);
    }

    #[test]
    fn closed_reopens_into_in_progress() {
        assert_eq!(apply_event(Closed, Reopen), InProgress);
    }

    #[test]
    fn cancelled_is_terminal() {
        assert_eq!(apply_event(Cancelled, Reopen), Cancelled);
        assert_eq!(apply_event(Cancelled, Start), Cancelled);
    }

    #[test]
    fn close_without_resolution_is_ignored() {
        assert_eq!(apply_event(InProgress, Close), InProgress);
        assert_eq!(apply_event(Blocked, Close), Blocked);
        assert_eq!(apply_event(New, Close), New);
    }

    #[test]
    fn realistic_lifecycle_with_reopen_and_reblock() {
        let events = [Start, Block, Unblock, Resolve, Reopen, Block, Resolve, Close];
        assert_eq!(fold_events(New, &events), Closed);
    }

    #[test]
    fn cancelled_midstream_stays_cancelled() {
        let events = [Start, Cancel, Reopen, Start, Resolve, Close];
        assert_eq!(fold_events(New, &events), Cancelled);
    }
}
