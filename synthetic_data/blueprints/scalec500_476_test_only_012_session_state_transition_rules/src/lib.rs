#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Idle,
    Active,
    Suspended,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Connect,
    Heartbeat,
    Suspend,
    Resume,
    Disconnect,
    ForceClose,
}

pub fn next_state(state: SessionState, event: Event) -> SessionState {
    use Event::*;
    use SessionState::*;

    match (state, event) {
        (Idle, Connect) => Active,
        (Idle, ForceClose) => Closed,

        (Active, Heartbeat) => Active,
        (Active, Suspend) => Suspended,
        (Active, Disconnect) => Idle,
        (Active, ForceClose) => Closed,

        (Suspended, Resume) => Active,
        (Suspended, Disconnect) => Idle,
        (Suspended, ForceClose) => Closed,

        (Closed, _) => Closed,

        (s, _) => s,
    }
}

pub fn apply_events(mut state: SessionState, events: &[Event]) -> SessionState {
    for &event in events {
        state = next_state(state, event);
    }
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_state_is_terminal() {
        let events = [Event::Connect, Event::ForceClose, Event::Resume, Event::Heartbeat];
        assert_eq!(apply_events(SessionState::Idle, &events), SessionState::Closed);
    }

    #[test]
    fn invalid_events_do_not_change_state() {
        assert_eq!(next_state(SessionState::Idle, Event::Resume), SessionState::Idle);
        assert_eq!(next_state(SessionState::Suspended, Event::Heartbeat), SessionState::Suspended);
    }

    #[test]
    fn suspend_then_resume_returns_to_active() {
        let events = [Event::Connect, Event::Suspend, Event::Resume];
        assert_eq!(apply_events(SessionState::Idle, &events), SessionState::Active);
    }

    #[test]
    fn disconnect_from_suspended_goes_idle_not_closed() {
        let events = [Event::Connect, Event::Suspend, Event::Disconnect, Event::Connect];
        assert_eq!(apply_events(SessionState::Idle, &events), SessionState::Active);
    }

    #[test]
    fn force_close_from_idle_skips_active() {
        assert_eq!(next_state(SessionState::Idle, Event::ForceClose), SessionState::Closed);
    }
}
