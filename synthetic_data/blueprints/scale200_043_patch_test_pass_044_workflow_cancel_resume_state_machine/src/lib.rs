#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Draft,
    Active,
    Paused,
    Cancelled,
    Completed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Submit,
    Pause,
    Resume,
    Cancel,
    Complete,
}

pub fn apply_events(events: &[Event]) -> Status {
    let mut status = Status::Draft;

    for event in events {
        status = match (status, event) {
            (Status::Draft, Event::Submit) => Status::Active,
            (Status::Active, Event::Pause) => Status::Paused,
            (Status::Paused, Event::Resume) => Status::Draft,
            (_, Event::Cancel) => Status::Cancelled,
            (Status::Active, Event::Complete) | (Status::Paused, Event::Complete) => Status::Completed,
            (s, _) => s,
        };
    }

    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event::*, Status};

    #[test]
    fn resume_returns_to_active_not_draft() {
        let events = [Submit, Pause, Resume];
        assert_eq!(apply_events(&events), Status::Active);
    }

    #[test]
    fn cancel_is_terminal_even_if_more_events_follow() {
        let events = [Submit, Pause, Cancel, Resume, Complete];
        assert_eq!(apply_events(&events), Status::Cancelled);
    }

    #[test]
    fn complete_only_from_active_or_paused() {
        assert_eq!(apply_events(&[Complete]), Status::Draft);
        assert_eq!(apply_events(&[Submit, Complete]), Status::Completed);
        assert_eq!(apply_events(&[Submit, Pause, Complete]), Status::Completed);
    }

    #[test]
    fn completed_is_terminal_even_if_cancel_comes_later() {
        let events = [Submit, Complete, Cancel];
        assert_eq!(apply_events(&events), Status::Completed);
    }

    #[test]
    fn duplicate_submit_does_not_change_active_workflow() {
        let events = [Submit, Submit, Pause, Resume];
        assert_eq!(apply_events(&events), Status::Active);
    }
}
