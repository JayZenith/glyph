#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TicketState {
    Draft,
    Open,
    InReview,
    Approved,
    Rejected,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Submit,
    StartReview,
    Approve,
    Reject,
    Reopen,
    Close,
}

pub fn apply_event(state: TicketState, event: Event) -> TicketState {
    use Event::*;
    use TicketState::*;

    match (state, event) {
        (Draft, Submit) => Open,
        (Open, StartReview) => InReview,
        (InReview, Approve) => Approved,
        (InReview, Reject) => Rejected,
        (Approved, Close) => Closed,
        (Rejected, Reopen) => Open,
        (Closed, Reopen) => Open,
        (s, _) => s,
    }
}

pub fn apply_events(initial: TicketState, events: &[Event]) -> TicketState {
    events.iter().fold(initial, |state, &event| apply_event(state, event))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path_reaches_closed() {
        let events = [
            Event::Submit,
            Event::StartReview,
            Event::Approve,
            Event::Close,
        ];
        assert_eq!(apply_events(TicketState::Draft, &events), TicketState::Closed);
    }

    #[test]
    fn reject_requires_reopen_before_reviewing_again() {
        let events = [
            Event::Submit,
            Event::StartReview,
            Event::Reject,
            Event::StartReview,
            Event::Reopen,
            Event::StartReview,
        ];
        assert_eq!(apply_events(TicketState::Draft, &events), TicketState::InReview);
    }

    #[test]
    fn close_only_works_after_approval() {
        let events = [Event::Submit, Event::Close, Event::StartReview, Event::Close];
        assert_eq!(apply_events(TicketState::Draft, &events), TicketState::InReview);
    }

    #[test]
    fn reopened_closed_ticket_can_be_reviewed_again() {
        let events = [
            Event::Submit,
            Event::StartReview,
            Event::Approve,
            Event::Close,
            Event::Reopen,
            Event::StartReview,
        ];
        assert_eq!(apply_events(TicketState::Draft, &events), TicketState::InReview);
    }

    #[test]
    fn invalid_events_leave_state_unchanged() {
        assert_eq!(apply_event(TicketState::Draft, Event::Approve), TicketState::Draft);
        assert_eq!(apply_event(TicketState::Approved, Event::Submit), TicketState::Approved);
        assert_eq!(apply_event(TicketState::Rejected, Event::Close), TicketState::Rejected);
    }
}
