#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParcelState {
    Pending,
    Packed,
    Shipped,
    Delivered,
    Returned,
    Lost,
}

pub fn customer_action(state: ParcelState, paid: bool) -> &'static str {
    match state {
        ParcelState::Pending => {
            if paid { "wait" } else { "pay_now" }
        }
        ParcelState::Packed => "track",
        ParcelState::Shipped => "track",
        ParcelState::Delivered => "track",
        ParcelState::Returned => "contact_support",
        ParcelState::Lost => "contact_support",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pending_depends_on_payment() {
        assert_eq!(customer_action(ParcelState::Pending, false), "pay_now");
        assert_eq!(customer_action(ParcelState::Pending, true), "wait");
    }

    #[test]
    fn transit_states_allow_tracking() {
        assert_eq!(customer_action(ParcelState::Packed, true), "track");
        assert_eq!(customer_action(ParcelState::Shipped, false), "track");
    }

    #[test]
    fn delivered_is_confirm_instead_of_track() {
        assert_eq!(customer_action(ParcelState::Delivered, true), "confirm_receipt");
    }

    #[test]
    fn exception_states_split_between_refund_and_support() {
        assert_eq!(customer_action(ParcelState::Returned, true), "await_refund");
        assert_eq!(customer_action(ParcelState::Lost, true), "contact_support");
    }
}
