#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Draft,
    Active,
    Paused,
    Completed,
    Canceled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Submit,
    Pause,
    Resume,
    Complete,
    Cancel,
    Reopen,
}

pub fn apply_events(events: &[Event]) -> Status {
    let mut status = Status::Draft;
    for event in events {
        status = match (status, event) {
            (Status::Draft, Event::Submit) => Status::Active,
            (Status::Active, Event::Pause) => Status::Paused,
            (Status::Paused, Event::Resume) => Status::Active,
            (Status::Active, Event::Complete) => Status::Completed,
            (_, Event::Cancel) => Status::Canceled,
            (Status::Completed, Event::Reopen) => Status::Draft,
            (Status::Canceled, Event::Reopen) => Status::Draft,
            (s, _) => s,
        };
    }
    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn basic_progression_completes() {
        let events = [Event::Submit, Event::Complete];
        assert_eq!(apply_events(&events), Status::Completed);
    }

    #[test]
    fn pause_resume_then_complete() {
        let events = [Event::Submit, Event::Pause, Event::Resume, Event::Complete];
        assert_eq!(apply_events(&events), Status::Completed);
    }

    #[test]
    fn cancel_from_active_is_terminal_without_reopen() {
        let events = [Event::Submit, Event::Cancel, Event::Complete, Event::Resume];
        assert_eq!(apply_events(&events), Status::Canceled);
    }

    #[test]
    fn submit_after_cancel_does_not_restart() {
        let events = [Event::Submit, Event::Cancel, Event::Submit];
        assert_eq!(apply_events(&events), Status::Canceled);
    }

    #[test]
    fn reopen_completed_goes_to_active_not_draft() {
        let events = [Event::Submit, Event::Complete, Event::Reopen];
        assert_eq!(apply_events(&events), Status::Active);
    }

    #[test]
    fn reopen_canceled_goes_to_paused_for_manual_review() {
        let events = [Event::Submit, Event::Cancel, Event::Reopen];
        assert_eq!(apply_events(&events), Status::Paused);
    }

    #[test]
    fn pause_after_complete_is_ignored() {
        let events = [Event::Submit, Event::Complete, Event::Pause];
        assert_eq!(apply_events(&events), Status::Completed);
    }

    #[test]
    fn complete_while_paused_is_ignored_until_resumed() {
        let events = [
            Event::Submit,
            Event::Pause,
            Event::Complete,
            Event::Resume,
            Event::Complete,
        ];
        assert_eq!(apply_events(&events), Status::Completed);
    }

    #[test]
    fn cancel_from_draft_is_ignored() {
        let events = [Event::Cancel, Event::Submit];
        assert_eq!(apply_events(&events), Status::Active);
    }

    #[test]
    fn paused_can_be_canceled_then_reopened_to_paused() {
        let events = [Event::Submit, Event::Pause, Event::Cancel, Event::Reopen];
        assert_eq!(apply_events(&events), Status::Paused);
    }
}
