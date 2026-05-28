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
        _ => state,
    }
}

pub fn apply_events(initial: State, events: &[Event]) -> State {
    let mut state = initial;
    for &event in events {
        state = apply_event(state, event);
    }
    state
}

#[cfg(test)]
mod tests {
    use super::{apply_event, apply_events, Event, State};

    #[test]
    fn happy_path_reaches_published() {
        let events = [Event::Submit, Event::Approve, Event::Publish];
        assert_eq!(apply_events(State::Draft, &events), State::Published);
    }

    #[test]
    fn rejected_items_can_reenter_review_directly() {
        let events = [Event::Submit, Event::Reject, Event::Revise, Event::Approve];
        assert_eq!(apply_events(State::Draft, &events), State::Approved);
    }

    #[test]
    fn published_items_are_terminal_except_archive() {
        assert_eq!(apply_event(State::Published, Event::Submit), State::Published);
        assert_eq!(apply_event(State::Published, Event::Approve), State::Published);
        assert_eq!(apply_event(State::Published, Event::Reject), State::Published);
        assert_eq!(apply_event(State::Published, Event::Revise), State::Published);
        assert_eq!(apply_event(State::Published, Event::Publish), State::Published);
        assert_eq!(apply_event(State::Published, Event::Restore), State::Published);
        assert_eq!(apply_event(State::Published, Event::Archive), State::Archived);
    }

    #[test]
    fn restore_preserves_completed_review() {
        let events = [Event::Submit, Event::Approve, Event::Publish, Event::Archive, Event::Restore, Event::Publish];
        assert_eq!(apply_events(State::Draft, &events), State::Published);
    }

    #[test]
    fn invalid_transitions_leave_state_unchanged() {
        assert_eq!(apply_event(State::Draft, Event::Approve), State::Draft);
        assert_eq!(apply_event(State::Approved, Event::Submit), State::Approved);
        assert_eq!(apply_event(State::Archived, Event::Approve), State::Archived);
    }
}
