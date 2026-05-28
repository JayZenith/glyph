#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryState {
    Created,
    Packed,
    InTransit,
    Delivered,
    Cancelled,
}

pub fn customer_message(state: DeliveryState, has_tracking: bool) -> &'static str {
    match state {
        DeliveryState::Created | DeliveryState::Packed => "preparing",
        DeliveryState::InTransit if has_tracking => "track shipment",
        DeliveryState::InTransit => "preparing",
        DeliveryState::Delivered => "delivered",
        DeliveryState::Cancelled => "cancelled",
    }
}

#[cfg(test)]
mod tests {
    use super::{customer_message, DeliveryState};

    #[test]
    fn created_and_packed_are_preparing() {
        assert_eq!(customer_message(DeliveryState::Created, false), "preparing");
        assert_eq!(customer_message(DeliveryState::Packed, true), "preparing");
    }

    #[test]
    fn in_transit_uses_tracking_when_available() {
        assert_eq!(customer_message(DeliveryState::InTransit, true), "track shipment");
    }

    #[test]
    fn in_transit_without_tracking_still_reports_transit() {
        assert_eq!(customer_message(DeliveryState::InTransit, false), "in transit");
    }

    #[test]
    fn terminal_states_have_specific_messages() {
        assert_eq!(customer_message(DeliveryState::Delivered, true), "delivered");
        assert_eq!(customer_message(DeliveryState::Cancelled, false), "cancelled");
    }
}
