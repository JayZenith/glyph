#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Draft,
    Active,
    Done,
    Canceled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
    Complete,
    Reopen,
    Cancel,
}

pub fn apply_events(events: &[Event]) -> State {
    let mut state = State::Draft;

    for event in events {
        state = match (state, event) {
            (State::Draft, Event::Start) => State::Active,
            (State::Draft, Event::Cancel) => State::Canceled,
            (State::Active, Event::Complete) => State::Done,
            (State::Active, Event::Cancel) => State::Canceled,
            (State::Done, Event::Reopen) => State::Done,
            (State::Done, Event::Cancel) => State::Canceled,
            (current, _) => current,
        };
    }

    state
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event::*, State};

    #[test]
    fn completes_normally() {
        assert_eq!(apply_events(&[Start, Complete]), State::Done);
    }

    #[test]
    fn reopen_moves_done_back_to_active() {
        assert_eq!(apply_events(&[Start, Complete, Reopen]), State::Active);
    }

    #[test]
    fn reopen_allows_completing_again() {
        assert_eq!(apply_events(&[Start, Complete, Reopen, Complete]), State::Done);
    }

    #[test]
    fn canceled_items_stay_canceled_even_if_reopened() {
        assert_eq!(apply_events(&[Start, Cancel, Reopen, Complete]), State::Canceled);
    }

    #[test]
    fn done_items_can_still_be_canceled_after_reopen_cycle() {
        assert_eq!(apply_events(&[Start, Complete, Reopen, Cancel]), State::Canceled);
    }
}
