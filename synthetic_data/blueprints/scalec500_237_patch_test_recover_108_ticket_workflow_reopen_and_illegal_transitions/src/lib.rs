#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Draft,
    Open,
    InProgress,
    Blocked,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Submit,
    Start,
    Block,
    Unblock,
    Resolve,
    Close,
    Reopen,
}

pub fn apply_events(events: &[Event]) -> Result<Status, String> {
    let mut status = Status::Draft;

    for &event in events {
        status = match (status, event) {
            (Status::Draft, Event::Submit) => Status::Open,
            (Status::Open, Event::Start) => Status::InProgress,
            (Status::InProgress, Event::Block) => Status::Blocked,
            (Status::Blocked, Event::Unblock) => Status::InProgress,
            (Status::InProgress, Event::Resolve) => Status::Resolved,
            (Status::Resolved, Event::Close) => Status::Closed,
            (Status::Closed, Event::Reopen) => Status::Open,
            (s, Event::Reopen) if s != Status::Closed => Status::Open,
            (s, _) => s,
        };
    }

    Ok(status)
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn happy_path_reaches_closed() {
        let events = [
            Event::Submit,
            Event::Start,
            Event::Resolve,
            Event::Close,
        ];
        assert_eq!(apply_events(&events).unwrap(), Status::Closed);
    }

    #[test]
    fn blocked_ticket_can_unblock_and_resolve() {
        let events = [
            Event::Submit,
            Event::Start,
            Event::Block,
            Event::Unblock,
            Event::Resolve,
        ];
        assert_eq!(apply_events(&events).unwrap(), Status::Resolved);
    }

    #[test]
    fn reopen_only_allowed_after_closed_and_goes_to_open() {
        let err = apply_events(&[Event::Reopen]).unwrap_err();
        assert!(err.contains("illegal transition"));

        let events = [
            Event::Submit,
            Event::Start,
            Event::Resolve,
            Event::Close,
            Event::Reopen,
        ];
        assert_eq!(apply_events(&events).unwrap(), Status::Open);
    }

    #[test]
    fn close_requires_resolved_first() {
        let err = apply_events(&[Event::Submit, Event::Close]).unwrap_err();
        assert!(err.contains("illegal transition"));
    }

    #[test]
    fn resolve_from_blocked_is_illegal() {
        let err = apply_events(&[
            Event::Submit,
            Event::Start,
            Event::Block,
            Event::Resolve,
        ])
        .unwrap_err();
        assert!(err.contains("illegal transition"));
    }
}
