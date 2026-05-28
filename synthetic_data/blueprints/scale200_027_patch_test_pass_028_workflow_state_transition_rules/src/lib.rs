#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Draft,
    Reviewing,
    Approved,
    Published,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Submit,
    Approve,
    Reject,
    Publish,
    Archive,
    Restore,
}

pub fn apply_events(events: &[Event]) -> State {
    let mut state = State::Draft;
    for event in events {
        state = match (state, *event) {
            (State::Draft, Event::Submit) => State::Reviewing,
            (State::Reviewing, Event::Approve) => State::Approved,
            (State::Reviewing, Event::Reject) => State::Draft,
            (State::Approved, Event::Publish) => State::Published,
            (State::Published, Event::Archive) => State::Archived,
            (State::Archived, Event::Restore) => State::Published,
            (State::Approved, Event::Archive) => State::Archived,
            (State::Archived, Event::Submit) => State::Reviewing,
            _ => state,
        };
    }
    state
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, State};

    #[test]
    fn happy_path_reaches_published() {
        let events = [Event::Submit, Event::Approve, Event::Publish];
        assert_eq!(apply_events(&events), State::Published);
    }

    #[test]
    fn reject_sends_review_back_to_draft() {
        let events = [Event::Submit, Event::Reject];
        assert_eq!(apply_events(&events), State::Draft);
    }

    #[test]
    fn approved_cannot_be_archived_before_publish() {
        let events = [Event::Submit, Event::Approve, Event::Archive];
        assert_eq!(apply_events(&events), State::Approved);
    }

    #[test]
    fn restore_returns_archived_item_to_draft() {
        let events = [
            Event::Submit,
            Event::Approve,
            Event::Publish,
            Event::Archive,
            Event::Restore,
        ];
        assert_eq!(apply_events(&events), State::Draft);
    }

    #[test]
    fn archived_item_does_not_resume_review_on_submit() {
        let events = [
            Event::Submit,
            Event::Approve,
            Event::Publish,
            Event::Archive,
            Event::Submit,
        ];
        assert_eq!(apply_events(&events), State::Archived);
    }

    #[test]
    fn invalid_events_leave_state_unchanged() {
        let events = [Event::Publish, Event::Approve, Event::Archive];
        assert_eq!(apply_events(&events), State::Draft);
    }
}
