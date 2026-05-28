#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Draft,
    Active,
    Paused,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Submit,
    Pause,
    Resume,
    Close,
    Reopen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    state: State,
    close_count: u8,
}

impl Ticket {
    pub fn new() -> Self {
        Self {
            state: State::Draft,
            close_count: 0,
        }
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn close_count(&self) -> u8 {
        self.close_count
    }

    pub fn apply(&mut self, event: Event) -> bool {
        match (self.state, event) {
            (State::Draft, Event::Submit) => {
                self.state = State::Active;
                true
            }
            (State::Active, Event::Pause) => {
                self.state = State::Paused;
                true
            }
            (State::Paused, Event::Resume) => {
                self.state = State::Active;
                true
            }
            (State::Active, Event::Close) | (State::Paused, Event::Close) => {
                self.state = State::Closed;
                self.close_count += 1;
                true
            }
            (State::Closed, Event::Reopen) if self.close_count < 2 => {
                self.state = State::Active;
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Event, State, Ticket};

    #[test]
    fn full_cycle_allows_single_reopen_then_final_close() {
        let mut t = Ticket::new();
        assert_eq!(t.state(), State::Draft);

        assert!(t.apply(Event::Submit));
        assert_eq!(t.state(), State::Active);

        assert!(t.apply(Event::Pause));
        assert_eq!(t.state(), State::Paused);

        assert!(t.apply(Event::Resume));
        assert_eq!(t.state(), State::Active);

        assert!(t.apply(Event::Close));
        assert_eq!(t.state(), State::Closed);
        assert_eq!(t.close_count(), 1);

        assert!(t.apply(Event::Reopen));
        assert_eq!(t.state(), State::Active);

        assert!(t.apply(Event::Close));
        assert_eq!(t.state(), State::Closed);
        assert_eq!(t.close_count(), 2);

        assert!(!t.apply(Event::Reopen));
        assert_eq!(t.state(), State::Closed);
    }

    #[test]
    fn invalid_events_do_not_change_state_or_counters() {
        let mut t = Ticket::new();

        assert!(!t.apply(Event::Pause));
        assert_eq!(t.state(), State::Draft);
        assert_eq!(t.close_count(), 0);

        assert!(t.apply(Event::Submit));
        assert!(!t.apply(Event::Resume));
        assert_eq!(t.state(), State::Active);
        assert_eq!(t.close_count(), 0);

        assert!(t.apply(Event::Close));
        assert_eq!(t.state(), State::Closed);
        assert_eq!(t.close_count(), 1);

        assert!(!t.apply(Event::Submit));
        assert!(!t.apply(Event::Pause));
        assert!(!t.apply(Event::Close));
        assert_eq!(t.state(), State::Closed);
        assert_eq!(t.close_count(), 1);
    }

    #[test]
    fn close_from_paused_counts_and_reopen_returns_to_active() {
        let mut t = Ticket::new();
        assert!(t.apply(Event::Submit));
        assert!(t.apply(Event::Pause));
        assert!(t.apply(Event::Close));

        assert_eq!(t.state(), State::Closed);
        assert_eq!(t.close_count(), 1);

        assert!(t.apply(Event::Reopen));
        assert_eq!(t.state(), State::Active);
    }
}
