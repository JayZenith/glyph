#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryStatus {
    Pending,
    InTransit { delayed: bool },
    Delivered,
    Failed { retryable: bool },
}

pub fn status_label(status: DeliveryStatus) -> &'static str {
    match status {
        DeliveryStatus::Pending => "pending",
        DeliveryStatus::InTransit { delayed } => {
            if delayed {
                "in transit"
            } else {
                "delayed"
            }
        }
        DeliveryStatus::Delivered => "delivered",
        DeliveryStatus::Failed { retryable } => {
            if retryable {
                "failed"
            } else {
                "retrying"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{status_label, DeliveryStatus};

    #[test]
    fn labels_pending_and_delivered() {
        assert_eq!(status_label(DeliveryStatus::Pending), "pending");
        assert_eq!(status_label(DeliveryStatus::Delivered), "delivered");
    }

    #[test]
    fn labels_in_transit_variants() {
        assert_eq!(
            status_label(DeliveryStatus::InTransit { delayed: false }),
            "in transit"
        );
        assert_eq!(
            status_label(DeliveryStatus::InTransit { delayed: true }),
            "delayed"
        );
    }

    #[test]
    fn labels_failed_variants() {
        assert_eq!(
            status_label(DeliveryStatus::Failed { retryable: true }),
            "retrying"
        );
        assert_eq!(
            status_label(DeliveryStatus::Failed { retryable: false }),
            "failed"
        );
    }
}
