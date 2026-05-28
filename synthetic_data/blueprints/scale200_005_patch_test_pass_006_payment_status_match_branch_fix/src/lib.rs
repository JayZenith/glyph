#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Payment {
    Pending,
    Authorized,
    Captured,
    Refunded,
    Failed,
}

pub fn next_action(state: Payment, amount_cents: u32) -> &'static str {
    match state {
        Payment::Pending => {
            if amount_cents == 0 {
                "reject"
            } else {
                "authorize"
            }
        }
        Payment::Authorized => {
            if amount_cents == 0 {
                "void"
            } else {
                "capture"
            }
        }
        Payment::Captured => {
            if amount_cents == 0 {
                "settle"
            } else {
                "refund"
            }
        }
        Payment::Refunded => "archive",
        Payment::Failed => "retry",
    }
}

#[cfg(test)]
mod tests {
    use super::{next_action, Payment};

    #[test]
    fn pending_zero_amount_is_rejected() {
        assert_eq!(next_action(Payment::Pending, 0), "reject");
    }

    #[test]
    fn pending_positive_amount_is_authorized() {
        assert_eq!(next_action(Payment::Pending, 150), "authorize");
    }

    #[test]
    fn authorized_zero_amount_is_voided() {
        assert_eq!(next_action(Payment::Authorized, 0), "void");
    }

    #[test]
    fn authorized_positive_amount_is_captured() {
        assert_eq!(next_action(Payment::Authorized, 150), "capture");
    }

    #[test]
    fn captured_state_always_settles() {
        assert_eq!(next_action(Payment::Captured, 0), "settle");
        assert_eq!(next_action(Payment::Captured, 150), "settle");
    }

    #[test]
    fn refunded_and_failed_have_terminal_actions() {
        assert_eq!(next_action(Payment::Refunded, 10), "archive");
        assert_eq!(next_action(Payment::Failed, 10), "retry");
    }
}
