#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    New,
    Active,
    Blocked,
    Done,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
    Block,
    Unblock,
    Complete,
    Cancel,
    Reopen,
}

pub fn apply_event(state: State, event: Event) -> State {
    match (state, event) {
        (State::New, Event::Start) => State::Active,
        (State::New, Event::Cancel) => State::Cancelled,
        (State::Active, Event::Block) => State::Blocked,
        (State::Active, Event::Complete) => State::Done,
        (State::Active, Event::Cancel) => State::Cancelled,
        (State::Blocked, Event::Unblock) => State::New,
        (State::Blocked, Event::Cancel) => State::Cancelled,
        (State::Done, Event::Reopen) => State::New,
        (State::Cancelled, Event::Reopen) => State::Active,
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
    use super::{apply_event, apply_events, Event::*, State::*};

    #[test]
    fn blocked_items_resume_active_not_new() {
        assert_eq!(apply_event(Blocked, Unblock), Active);
    }

    #[test]
    fn reopening_done_goes_to_active() {
        assert_eq!(apply_event(Done, Reopen), Active);
    }

    #[test]
    fn reopening_cancelled_requires_restart_from_new() {
        assert_eq!(apply_event(Cancelled, Reopen), New);
    }

    #[test]
    fn complex_flow_honors_intermediate_states() {
        let events = [Start, Block, Unblock, Complete, Reopen, Cancel];
        assert_eq!(apply_events(New, &events), Cancelled);
    }

    #[test]
    fn invalid_transitions_leave_state_unchanged() {
        assert_eq!(apply_event(New, Complete), New);
        assert_eq!(apply_event(Done, Block), Done);
        assert_eq!(apply_event(Cancelled, Complete), Cancelled);
    }
}
