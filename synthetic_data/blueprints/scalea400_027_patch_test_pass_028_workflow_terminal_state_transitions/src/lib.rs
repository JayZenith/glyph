#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Draft,
    Review,
    Approved,
    Rejected,
    Published,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Submit,
    Approve,
    Reject,
    Revise,
    Publish,
    Archive,
    Restore,
}

pub fn apply_event(state: State, event: Event) -> State {
    use Event::*;
    use State::*;

    match (state, event) {
        (Draft, Submit) => Review,
        (Review, Approve) => Approved,
        (Review, Reject) => Rejected,
        (Rejected, Revise) => Draft,
        (Approved, Publish) => Published,
        (Published, Archive) => Archived,
        (Archived, Restore) => Draft,
        (Rejected, Submit) => Review,
        (Approved, Reject) => Rejected,
        _ => state,
    }
}

pub fn apply_events(mut state: State, events: &[Event]) -> State {
    for &event in events {
        state = apply_event(state, event);
    }
    state
}

#[cfg(test)]
mod tests {
    use super::{apply_event, apply_events, Event::*, State::*};

    #[test]
    fn happy_path_reaches_published() {
        let end = apply_events(Draft, &[Submit, Approve, Publish]);
        assert_eq!(end, Published);
    }

    #[test]
    fn rejection_requires_revision_before_resubmission() {
        let end = apply_events(Draft, &[Submit, Reject, Submit]);
        assert_eq!(end, Rejected);
    }

    #[test]
    fn approved_items_cannot_be_rejected() {
        assert_eq!(apply_event(Approved, Reject), Approved);
    }

    #[test]
    fn archived_restore_returns_to_review_not_draft() {
        assert_eq!(apply_event(Archived, Restore), Review);
    }

    #[test]
    fn invalid_events_leave_terminal_states_unchanged() {
        let end = apply_events(Published, &[Submit, Approve, Reject, Revise]);
        assert_eq!(end, Published);
    }

    #[test]
    fn archive_and_restore_allows_republish_without_resubmit() {
        let end = apply_events(Draft, &[Submit, Approve, Publish, Archive, Restore, Approve, Publish]);
        assert_eq!(end, Published);
    }
}
