#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Open,
    InProgress,
    Blocked,
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

pub fn apply_event(state: State, event: Event) -> State {
    match (state, event) {
        (State::Open, Event::Start) => State::InProgress,
        (State::InProgress, Event::Block) => State::Blocked,
        (State::Blocked, Event::Resume) => State::InProgress,
        (State::Open, Event::Close)
        | (State::InProgress, Event::Close)
        | (State::Blocked, Event::Close) => State::Closed,
        (State::Closed, Event::Reopen) => State::Open,
        _ => state,
    }
}

pub fn apply_all(mut state: State, events: &[Event]) -> State {
    for &event in events {
        state = apply_event(state, event);
    }
    state
}

#[cfg(test)]
mod tests {
    use super::{apply_all, apply_event, Event, State};

    #[test]
    fn progresses_through_normal_flow() {
        let final_state = apply_all(State::Open, &[Event::Start, Event::Close]);
        assert_eq!(final_state, State::Closed);
    }

    #[test]
    fn blocked_work_can_resume() {
        let final_state = apply_all(
            State::Open,
            &[Event::Start, Event::Block, Event::Resume, Event::Close],
        );
        assert_eq!(final_state, State::Closed);
    }

    #[test]
    fn reopen_only_changes_closed_items() {
        assert_eq!(apply_event(State::Closed, Event::Reopen), State::Open);
        assert_eq!(apply_event(State::InProgress, Event::Reopen), State::InProgress);
    }

    #[test]
    fn invalid_transitions_leave_state_unchanged() {
        assert_eq!(apply_event(State::Open, Event::Resume), State::Open);
        assert_eq!(apply_event(State::Closed, Event::Block), State::Closed);
    }
}
