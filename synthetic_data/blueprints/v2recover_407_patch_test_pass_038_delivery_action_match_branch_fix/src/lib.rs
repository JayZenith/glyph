#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delivery {
    Draft,
    Queued { attempts: u8 },
    Sent,
    Failed { permanent: bool },
}

pub fn action_for(state: Delivery) -> &'static str {
    match state {
        Delivery::Draft => "queue",
        Delivery::Queued { attempts } if attempts >= 3 => "retry",
        Delivery::Queued { .. } => "wait",
        Delivery::Sent => "archive",
        Delivery::Failed { permanent: true } => "retry",
        Delivery::Failed { permanent: false } => "drop",
    }
}

#[cfg(test)]
mod tests {
    use super::{action_for, Delivery};

    #[test]
    fn draft_is_queued() {
        assert_eq!(action_for(Delivery::Draft), "queue");
    }

    #[test]
    fn queued_with_few_attempts_waits() {
        assert_eq!(action_for(Delivery::Queued { attempts: 2 }), "wait");
    }

    #[test]
    fn queued_at_retry_threshold_is_retried() {
        assert_eq!(action_for(Delivery::Queued { attempts: 3 }), "retry");
    }

    #[test]
    fn sent_is_archived() {
        assert_eq!(action_for(Delivery::Sent), "archive");
    }

    #[test]
    fn temporary_failure_is_retried() {
        assert_eq!(action_for(Delivery::Failed { permanent: false }), "retry");
    }

    #[test]
    fn permanent_failure_is_dropped() {
        assert_eq!(action_for(Delivery::Failed { permanent: true }), "drop");
    }
}
