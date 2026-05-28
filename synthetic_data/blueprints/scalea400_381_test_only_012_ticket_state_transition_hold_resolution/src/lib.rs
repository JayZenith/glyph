#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Open,
    InProgress,
    OnHold,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    StartWork,
    Pause,
    Resume,
    Resolve,
    Close,
    Reopen,
}

pub fn apply_events(mut status: Status, events: &[Event]) -> Status {
    for event in events {
        status = match (status, *event) {
            (Status::Open, Event::StartWork) => Status::InProgress,
            (Status::Open, Event::Close) => Status::Closed,

            (Status::InProgress, Event::Pause) => Status::OnHold,
            (Status::InProgress, Event::Resolve) => Status::Resolved,

            (Status::OnHold, Event::Resume) => Status::InProgress,
            (Status::OnHold, Event::Resolve) => Status::OnHold,

            (Status::Resolved, Event::Close) => Status::Closed,
            (Status::Resolved, Event::Reopen) => Status::Open,

            (Status::Closed, Event::Reopen) => Status::Open,

            (s, _) => s,
        };
    }
    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event::*, Status::*};

    #[test]
    fn basic_progression_to_closed() {
        let events = [StartWork, Resolve, Close];
        assert_eq!(apply_events(Open, &events), Closed);
    }

    #[test]
    fn hold_must_resume_before_resolve() {
        let events = [StartWork, Pause, Resolve, Resume, Resolve];
        assert_eq!(apply_events(Open, &events), Resolved);
    }

    #[test]
    fn direct_resolve_while_on_hold_does_nothing() {
        let events = [StartWork, Pause, Resolve];
        assert_eq!(apply_events(Open, &events), OnHold);
    }

    #[test]
    fn reopen_from_closed_requires_work_again() {
        let events = [StartWork, Resolve, Close, Reopen, Resolve, StartWork, Resolve];
        assert_eq!(apply_events(Open, &events), Resolved);
    }

    #[test]
    fn reopen_from_resolved_goes_back_to_open() {
        let events = [StartWork, Resolve, Reopen];
        assert_eq!(apply_events(Open, &events), Open);
    }
}
