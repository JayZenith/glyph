#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Open,
    InProgress,
    Blocked,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
    Block,
    Unblock,
    Resolve,
    Close,
    Reopen,
}

pub fn advance(status: Status, event: Event) -> Status {
    use Event::*;
    use Status::*;

    match (status, event) {
        (Open, Start) => InProgress,
        (InProgress, Block) => Blocked,
        (Blocked, Unblock) => InProgress,
        (InProgress, Resolve) | (Blocked, Resolve) => Resolved,
        (Resolved, Close) => Closed,
        (_, Reopen) => Open,
        _ => status,
    }
}

pub fn apply_events(initial: Status, events: &[Event]) -> Status {
    let mut status = initial;
    for &event in events {
        status = advance(status, event);
    }
    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event::*, Status::*};

    #[test]
    fn resolved_reopens_to_in_progress() {
        let end = apply_events(Resolved, &[Reopen]);
        assert_eq!(end, InProgress);
    }

    #[test]
    fn closed_reopens_to_in_progress_then_can_resolve_again() {
        let end = apply_events(Closed, &[Reopen, Resolve]);
        assert_eq!(end, Resolved);
    }

    #[test]
    fn unrelated_reopen_from_open_is_ignored() {
        let end = apply_events(Open, &[Reopen, Start]);
        assert_eq!(end, InProgress);
    }

    #[test]
    fn blocked_task_can_reopen_only_after_resolution_path() {
        let end = apply_events(Open, &[Start, Block, Reopen, Unblock, Resolve, Close, Reopen]);
        assert_eq!(end, InProgress);
    }

    #[test]
    fn invalid_close_from_in_progress_is_still_ignored() {
        let end = apply_events(Open, &[Start, Close, Resolve]);
        assert_eq!(end, Resolved);
    }
}
