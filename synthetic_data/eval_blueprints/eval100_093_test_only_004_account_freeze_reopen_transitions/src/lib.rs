#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Active,
    Frozen,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Freeze,
    Reopen,
    Close,
    Chargeback,
}

pub fn apply_events(events: &[Event]) -> Status {
    let mut status = Status::Active;
    for event in events {
        status = match (status, *event) {
            (Status::Active, Event::Freeze) => Status::Frozen,
            (Status::Active, Event::Close) => Status::Closed,
            (Status::Active, Event::Chargeback) => Status::Frozen,
            (Status::Active, Event::Reopen) => Status::Active,

            (Status::Frozen, Event::Reopen) => Status::Active,
            (Status::Frozen, Event::Close) => Status::Closed,
            (Status::Frozen, Event::Freeze) => Status::Frozen,
            (Status::Frozen, Event::Chargeback) => Status::Frozen,

            (Status::Closed, Event::Reopen) => Status::Active,
            (Status::Closed, Event::Close) => Status::Closed,
            (Status::Closed, Event::Freeze) => Status::Closed,
            (Status::Closed, Event::Chargeback) => Status::Closed,
        };
    }
    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event::*, Status};

    #[test]
    fn close_is_terminal_until_reopen() {
        let events = [Close, Freeze, Chargeback];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn reopen_after_close_restores_active() {
        let events = [Freeze, Close, Reopen];
        assert_eq!(apply_events(&events), Status::Active);
    }

    #[test]
    fn chargeback_freezes_but_does_not_close() {
        let events = [Chargeback];
        assert_eq!(apply_events(&events), Status::Frozen);
    }

    #[test]
    fn repeated_freeze_keeps_frozen() {
        let events = [Freeze, Freeze, Freeze];
        assert_eq!(apply_events(&events), Status::Frozen);
    }

    #[test]
    fn reopen_from_active_is_noop() {
        let events = [Reopen, Freeze];
        assert_eq!(apply_events(&events), Status::Frozen);
    }

    #[test]
    fn closed_then_reopen_then_chargeback_ends_frozen() {
        let events = [Close, Reopen, Chargeback];
        assert_eq!(apply_events(&events), Status::Frozen);
    }
}
