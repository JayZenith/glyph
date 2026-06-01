#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderState {
    Draft,
    Submitted,
    Approved,
    Rejected,
    Archived,
}

pub fn next_state(state: OrderState, event: &str) -> OrderState {
    match (state, event) {
        (OrderState::Draft, "submit") => OrderState::Submitted,
        (OrderState::Submitted, "approve") => OrderState::Approved,
        (OrderState::Submitted, "reject") => OrderState::Rejected,
        (OrderState::Approved, "archive") => OrderState::Archived,
        (OrderState::Rejected, "resubmit") => OrderState::Submitted,
        (OrderState::Archived, "reopen") => OrderState::Submitted,
        _ => state,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_flow_transitions() {
        assert_eq!(next_state(OrderState::Draft, "submit"), OrderState::Submitted);
        assert_eq!(next_state(OrderState::Submitted, "approve"), OrderState::Approved);
        assert_eq!(next_state(OrderState::Approved, "archive"), OrderState::Archived);
    }

    #[test]
    fn rejected_orders_can_be_reworked_from_draft() {
        assert_eq!(next_state(OrderState::Submitted, "reject"), OrderState::Rejected);
        assert_eq!(next_state(OrderState::Rejected, "resubmit"), OrderState::Draft);
    }

    #[test]
    fn archived_orders_reopen_as_draft() {
        assert_eq!(next_state(OrderState::Archived, "reopen"), OrderState::Draft);
    }

    #[test]
    fn unsupported_events_leave_state_unchanged() {
        assert_eq!(next_state(OrderState::Draft, "approve"), OrderState::Draft);
        assert_eq!(next_state(OrderState::Approved, "submit"), OrderState::Approved);
    }
}
