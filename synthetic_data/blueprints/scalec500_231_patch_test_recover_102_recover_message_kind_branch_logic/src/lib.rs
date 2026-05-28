#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Low,
    Normal,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageKind {
    Email,
    Sms,
    Push,
    Webhook,
}

pub fn classify(kind: MessageKind, urgent: bool, retries: u8) -> (&'static str, Priority) {
    match kind {
        MessageKind::Email => {
            if urgent {
                ("fast-lane", Priority::High)
            } else {
                ("digest", Priority::Low)
            }
        }
        MessageKind::Sms => {
            if retries > 0 {
                ("retry", Priority::Normal)
            } else {
                ("transactional", Priority::Normal)
            }
        }
        MessageKind::Push => {
            if urgent {
                ("alert", Priority::High)
            } else {
                ("bulk", Priority::Low)
            }
        }
        MessageKind::Webhook => {
            if retries > 3 {
                ("dead-letter", Priority::Low)
            } else {
                ("fire-and-forget", Priority::Normal)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_urgent_is_high_priority_fast_lane() {
        assert_eq!(classify(MessageKind::Email, true, 0), ("fast-lane", Priority::High));
    }

    #[test]
    fn email_non_urgent_is_digest_normal_priority() {
        assert_eq!(classify(MessageKind::Email, false, 0), ("digest", Priority::Normal));
    }

    #[test]
    fn sms_urgent_is_transactional_and_high_priority() {
        assert_eq!(classify(MessageKind::Sms, true, 0), ("transactional", Priority::High));
    }

    #[test]
    fn sms_retry_stays_retry_but_escalates_when_urgent() {
        assert_eq!(classify(MessageKind::Sms, true, 2), ("retry", Priority::High));
    }

    #[test]
    fn sms_retry_non_urgent_keeps_normal_priority() {
        assert_eq!(classify(MessageKind::Sms, false, 1), ("retry", Priority::Normal));
    }

    #[test]
    fn push_non_urgent_is_bulk_low_priority() {
        assert_eq!(classify(MessageKind::Push, false, 0), ("bulk", Priority::Low));
    }

    #[test]
    fn webhook_many_retries_goes_dead_letter_high_priority() {
        assert_eq!(classify(MessageKind::Webhook, false, 4), ("dead-letter", Priority::High));
    }

    #[test]
    fn webhook_urgent_without_many_retries_is_callback_high_priority() {
        assert_eq!(classify(MessageKind::Webhook, true, 1), ("callback", Priority::High));
    }

    #[test]
    fn webhook_non_urgent_without_many_retries_is_callback_normal_priority() {
        assert_eq!(classify(MessageKind::Webhook, false, 2), ("callback", Priority::Normal));
    }
}
