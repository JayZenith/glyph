#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TicketState {
    New,
    InProgress,
    Resolved,
    Closed,
}

pub fn apply_event(state: TicketState, event: &str) -> TicketState {
    match (state, event) {
        (TicketState::New, "start") => TicketState::InProgress,
        (TicketState::InProgress, "resolve") => TicketState::Closed,
        (TicketState::Resolved, "close") => TicketState::Resolved,
        (_, "reopen") => TicketState::New,
        _ => state,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_flow_reaches_closed() {
        let mut state = TicketState::New;
        state = apply_event(state, "start");
        state = apply_event(state, "resolve");
        state = apply_event(state, "close");
        assert_eq!(state, TicketState::Closed);
    }

    #[test]
    fn reopen_from_resolved_goes_back_to_in_progress() {
        let state = apply_event(TicketState::Resolved, "reopen");
        assert_eq!(state, TicketState::InProgress);
    }

    #[test]
    fn invalid_event_keeps_state() {
        let state = apply_event(TicketState::Closed, "start");
        assert_eq!(state, TicketState::Closed);
    }
}
