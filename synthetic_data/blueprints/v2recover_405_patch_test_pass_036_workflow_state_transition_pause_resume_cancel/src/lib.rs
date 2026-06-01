#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    Running,
    Paused,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
    Pause,
    Resume,
    Finish,
    Cancel,
    Reset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Machine {
    state: State,
    completed_runs: u32,
    pause_count: u32,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            state: State::Idle,
            completed_runs: 0,
            pause_count: 0,
        }
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn completed_runs(&self) -> u32 {
        self.completed_runs
    }

    pub fn pause_count(&self) -> u32 {
        self.pause_count
    }

    pub fn apply(&mut self, event: Event) -> bool {
        match (self.state, event) {
            (State::Idle, Event::Start) => {
                self.state = State::Running;
                true
            }
            (State::Running, Event::Pause) => {
                self.state = State::Paused;
                self.pause_count += 1;
                true
            }
            (State::Paused, Event::Resume) => {
                self.state = State::Running;
                self.pause_count += 1;
                true
            }
            (State::Running, Event::Finish) => {
                self.state = State::Completed;
                self.completed_runs += 1;
                true
            }
            (_, Event::Cancel) => {
                self.state = State::Cancelled;
                true
            }
            (_, Event::Reset) => {
                self.state = State::Idle;
                self.pause_count = 0;
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pause_resume_finish_tracks_counts() {
        let mut m = Machine::new();
        assert!(m.apply(Event::Start));
        assert!(m.apply(Event::Pause));
        assert_eq!(m.state(), State::Paused);
        assert_eq!(m.pause_count(), 1);

        assert!(m.apply(Event::Resume));
        assert_eq!(m.state(), State::Running);
        assert_eq!(m.pause_count(), 1);

        assert!(m.apply(Event::Finish));
        assert_eq!(m.state(), State::Completed);
        assert_eq!(m.completed_runs(), 1);
    }

    #[test]
    fn cancel_is_only_valid_while_active() {
        let mut m = Machine::new();
        assert!(!m.apply(Event::Cancel));
        assert_eq!(m.state(), State::Idle);

        assert!(m.apply(Event::Start));
        assert!(m.apply(Event::Cancel));
        assert_eq!(m.state(), State::Cancelled);

        assert!(!m.apply(Event::Cancel));
        assert_eq!(m.state(), State::Cancelled);
    }

    #[test]
    fn reset_clears_transient_state_but_preserves_history() {
        let mut m = Machine::new();
        assert!(m.apply(Event::Start));
        assert!(m.apply(Event::Pause));
        assert!(m.apply(Event::Resume));
        assert!(m.apply(Event::Finish));
        assert_eq!(m.completed_runs(), 1);
        assert_eq!(m.pause_count(), 1);

        assert!(m.apply(Event::Reset));
        assert_eq!(m.state(), State::Idle);
        assert_eq!(m.completed_runs(), 1);
        assert_eq!(m.pause_count(), 0);

        assert!(m.apply(Event::Start));
        assert!(m.apply(Event::Finish));
        assert_eq!(m.completed_runs(), 2);
    }

    #[test]
    fn completed_requires_reset_before_restart() {
        let mut m = Machine::new();
        assert!(m.apply(Event::Start));
        assert!(m.apply(Event::Finish));
        assert_eq!(m.state(), State::Completed);

        assert!(!m.apply(Event::Start));
        assert_eq!(m.state(), State::Completed);

        assert!(m.apply(Event::Reset));
        assert_eq!(m.state(), State::Idle);
        assert!(m.apply(Event::Start));
        assert_eq!(m.state(), State::Running);
    }
}
