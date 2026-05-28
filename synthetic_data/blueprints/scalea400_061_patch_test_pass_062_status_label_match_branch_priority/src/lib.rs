#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delivery {
    Immediate,
    Deferred,
    Suppressed,
}

pub fn action_label(severity: Severity, delivery: Delivery) -> &'static str {
    match delivery {
        Delivery::Immediate => "send-now",
        Delivery::Deferred => "queue",
        Delivery::Suppressed => match severity {
            Severity::Error => "drop",
            Severity::Warning => "queue",
            Severity::Info => "ignore",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn immediate_error_escalates() {
        assert_eq!(action_label(Severity::Error, Delivery::Immediate), "page-now");
    }

    #[test]
    fn immediate_non_error_still_sends_now() {
        assert_eq!(action_label(Severity::Warning, Delivery::Immediate), "send-now");
        assert_eq!(action_label(Severity::Info, Delivery::Immediate), "send-now");
    }

    #[test]
    fn deferred_error_is_queued_for_retry() {
        assert_eq!(action_label(Severity::Error, Delivery::Deferred), "retry");
    }

    #[test]
    fn deferred_non_error_uses_normal_queue() {
        assert_eq!(action_label(Severity::Warning, Delivery::Deferred), "queue");
        assert_eq!(action_label(Severity::Info, Delivery::Deferred), "queue");
    }

    #[test]
    fn suppressed_warning_and_info_dont_queue() {
        assert_eq!(action_label(Severity::Warning, Delivery::Suppressed), "suppress");
        assert_eq!(action_label(Severity::Info, Delivery::Suppressed), "ignore");
    }

    #[test]
    fn suppressed_error_is_still_dropped() {
        assert_eq!(action_label(Severity::Error, Delivery::Suppressed), "drop");
    }
}
