#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryState {
    Pending,
    Packed,
    Shipped { express: bool },
    Returned { damaged: bool },
    Cancelled,
}

pub fn customer_label(state: DeliveryState) -> &'static str {
    match state {
        DeliveryState::Pending => "processing",
        DeliveryState::Packed => "ready",
        DeliveryState::Shipped { express: true } => "shipped",
        DeliveryState::Shipped { express: false } => "out for delivery",
        DeliveryState::Returned { damaged: true } => "returned",
        DeliveryState::Returned { damaged: false } => "damaged return",
        DeliveryState::Cancelled => "cancelled",
    }
}

#[cfg(test)]
mod tests {
    use super::{customer_label, DeliveryState};

    #[test]
    fn labels_pending_and_packed() {
        assert_eq!(customer_label(DeliveryState::Pending), "processing");
        assert_eq!(customer_label(DeliveryState::Packed), "ready");
    }

    #[test]
    fn labels_shipped_variants() {
        assert_eq!(
            customer_label(DeliveryState::Shipped { express: true }),
            "out for delivery"
        );
        assert_eq!(
            customer_label(DeliveryState::Shipped { express: false }),
            "shipped"
        );
    }

    #[test]
    fn labels_returned_variants() {
        assert_eq!(
            customer_label(DeliveryState::Returned { damaged: true }),
            "damaged return"
        );
        assert_eq!(
            customer_label(DeliveryState::Returned { damaged: false }),
            "returned"
        );
    }

    #[test]
    fn label_cancelled() {
        assert_eq!(customer_label(DeliveryState::Cancelled), "cancelled");
    }
}
