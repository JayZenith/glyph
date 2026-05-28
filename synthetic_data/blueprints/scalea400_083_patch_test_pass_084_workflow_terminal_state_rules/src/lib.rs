#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Draft,
    Active,
    Paused,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Submit,
    Pause,
    Resume,
    Complete,
    Cancel,
}

pub fn apply_event(state: State, event: Event) -> State {
    match (state, event) {
        (State::Draft, Event::Submit) => State::Active,
        (State::Active, Event::Pause) => State::Paused,
        (State::Paused, Event::Resume) => State::Active,
        (State::Active, Event::Complete) | (State::Paused, Event::Complete) => State::Completed,
        (_, Event::Cancel) => State::Cancelled,
        _ => state,
    }
}

pub fn apply_events(mut state: State, events: &[Event]) -> State {
    for &event in events {
        state = apply_event(state, event);
    }
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path_reaches_completed() {
        let events = [Event::Submit, Event::Pause, Event::Resume, Event::Complete];
        assert_eq!(apply_events(State::Draft, &events), State::Completed);
    }

    #[test]
    fn completed_is_terminal_even_if_cancel_arrives_later() {
        let events = [Event::Submit, Event::Complete, Event::Cancel];
        assert_eq!(apply_events(State::Draft, &events), State::Completed);
    }

    #[test]
    fn cancelled_is_terminal_even_if_submit_arrives_later() {
        let events = [Event::Cancel, Event::Submit, Event::Complete];
        assert_eq!(apply_events(State::Draft, &events), State::Cancelled);
    }

    #[test]
    fn invalid_events_leave_state_unchanged() {
        let events = [Event::Resume, Event::Pause, Event::Submit, Event::Submit];
        assert_eq!(apply_events(State::Draft, &events), State::Active);
    }
}
