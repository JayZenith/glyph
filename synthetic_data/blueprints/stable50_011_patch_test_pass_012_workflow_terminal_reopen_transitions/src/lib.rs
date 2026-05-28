#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    New,
    Active,
    Paused,
    Closed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
    Pause,
    Resume,
    Close,
    Cancel,
    Reopen,
}

pub fn apply_events(events: &[Event]) -> Status {
    let mut status = Status::New;

    for event in events {
        status = match (status, *event) {
            (Status::New, Event::Start) => Status::Active,
            (Status::New, Event::Cancel) => Status::Cancelled,
            (Status::Active, Event::Pause) => Status::Paused,
            (Status::Active, Event::Close) => Status::Closed,
            (Status::Active, Event::Cancel) => Status::Cancelled,
            (Status::Paused, Event::Resume) => Status::Active,
            (Status::Paused, Event::Close) => Status::Closed,
            (Status::Paused, Event::Cancel) => Status::Cancelled,
            (Status::Closed, Event::Reopen) => Status::Active,
            (Status::Cancelled, Event::Reopen) => Status::Active,
            _ => status,
        };
    }

    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn cancel_is_terminal() {
        let events = [Event::Start, Event::Cancel, Event::Reopen, Event::Close];
        assert_eq!(apply_events(&events), Status::Cancelled);
    }

    #[test]
    fn close_can_reopen_but_needs_restart_before_pause_or_close() {
        let events = [
            Event::Start,
            Event::Close,
            Event::Pause,
            Event::Close,
            Event::Start,
            Event::Pause,
            Event::Resume,
            Event::Close,
        ];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn reopen_after_close_returns_to_new_not_active() {
        let events = [Event::Start, Event::Pause, Event::Resume, Event::Close, Event::Reopen];
        assert_eq!(apply_events(&events), Status::New);
    }

    #[test]
    fn paused_reopen_is_ignored_and_cancel_from_new_stays_cancelled() {
        let events = [Event::Cancel, Event::Reopen, Event::Start, Event::Pause, Event::Reopen];
        assert_eq!(apply_events(&events), Status::Cancelled);
    }
}
