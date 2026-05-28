#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    New,
    Active,
    Waiting,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
    Block,
    Resume,
    Close,
    Reopen,
}

pub fn apply_events(events: &[Event]) -> Status {
    let mut status = Status::New;
    for event in events {
        status = match (status, *event) {
            (Status::New, Event::Start) => Status::Active,
            (Status::Active, Event::Block) => Status::Waiting,
            (Status::Waiting, Event::Resume) => Status::Active,
            (_, Event::Close) => Status::Closed,
            (Status::Closed, Event::Reopen) => Status::New,
            _ => status,
        };
    }
    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn block_then_resume_round_trip() {
        let events = [Event::Start, Event::Block, Event::Resume];
        assert_eq!(apply_events(&events), Status::Active);
    }

    #[test]
    fn reopen_keeps_progress_ready_to_work() {
        let events = [Event::Start, Event::Close, Event::Reopen];
        assert_eq!(apply_events(&events), Status::Active);
    }

    #[test]
    fn close_from_new_is_ignored() {
        let events = [Event::Close, Event::Start];
        assert_eq!(apply_events(&events), Status::Active);
    }

    #[test]
    fn reopen_from_waiting_does_nothing() {
        let events = [Event::Start, Event::Block, Event::Reopen];
        assert_eq!(apply_events(&events), Status::Waiting);
    }
}
