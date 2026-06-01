#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryState {
    Pending,
    Packed,
    Shipped,
    Delivered,
    Returned,
    Lost,
}

pub fn next_action(state: DeliveryState, priority: bool) -> &'static str {
    match state {
        DeliveryState::Pending => {
            if priority {
                "rush-pick"
            } else {
                "queue-pick"
            }
        }
        DeliveryState::Packed => "print-label",
        DeliveryState::Shipped => {
            if priority {
                "call-carrier"
            } else {
                "wait-tracking"
            }
        }
        DeliveryState::Delivered => "confirm-delivery",
        DeliveryState::Returned => "close-case",
        DeliveryState::Lost => "close-case",
    }
}

#[cfg(test)]
mod tests {
    use super::{next_action, DeliveryState::*};

    #[test]
    fn pending_depends_on_priority() {
        assert_eq!(next_action(Pending, false), "queue-pick");
        assert_eq!(next_action(Pending, true), "rush-pick");
    }

    #[test]
    fn packed_and_shipped_are_distinct() {
        assert_eq!(next_action(Packed, false), "schedule-dispatch");
        assert_eq!(next_action(Packed, true), "schedule-dispatch");
        assert_eq!(next_action(Shipped, false), "wait-tracking");
        assert_eq!(next_action(Shipped, true), "expedite-trace");
    }

    #[test]
    fn terminal_states_have_specific_actions() {
        assert_eq!(next_action(Delivered, false), "archive-proof");
        assert_eq!(next_action(Returned, true), "inspect-return");
        assert_eq!(next_action(Lost, false), "open-claim");
    }
}
