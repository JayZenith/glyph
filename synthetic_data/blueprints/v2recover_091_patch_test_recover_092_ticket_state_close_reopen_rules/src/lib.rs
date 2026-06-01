#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    New,
    InProgress,
    Blocked,
    Closed,
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Start,
    Block,
    Unblock,
    Close,
    Reopen,
}

pub fn apply_events(events: &[Event]) -> Status {
    let mut status = Status::New;
    for event in events {
        status = match (*event, status) {
            (Event::Start, Status::New) => Status::InProgress,
            (Event::Block, Status::InProgress) => Status::Blocked,
            (Event::Unblock, Status::Blocked) => Status::InProgress,
            (Event::Close, _) => Status::Closed,
            (Event::Reopen, Status::Closed) => Status::New,
            _ => status,
        };
    }
    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn close_requires_active_work() {
        assert_eq!(apply_events(&[Event::Close]), Status::New);
        assert_eq!(apply_events(&[Event::Start, Event::Close]), Status::Closed);
        assert_eq!(apply_events(&[Event::Start, Event::Block, Event::Close]), Status::Closed);
    }

    #[test]
    fn reopen_goes_back_to_in_progress() {
        assert_eq!(
            apply_events(&[Event::Start, Event::Close, Event::Reopen]),
            Status::InProgress
        );
    }

    #[test]
    fn invalid_events_do_not_change_closed_ticket() {
        assert_eq!(
            apply_events(&[Event::Start, Event::Close, Event::Block, Event::Unblock]),
            Status::Closed
        );
    }
}
