#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Email,
    Sms,
    Push,
    Webhook,
}

pub fn audit_label(channel: Channel, urgent: bool, verified: bool) -> &'static str {
    match channel {
        Channel::Email => {
            if urgent {
                "email-priority"
            } else {
                "email-standard"
            }
        }
        Channel::Sms => {
            if verified {
                "sms-direct"
            } else {
                "sms-pending"
            }
        }
        Channel::Push => {
            if urgent || verified {
                "push-alert"
            } else {
                "push-bulk"
            }
        }
        Channel::Webhook => {
            if urgent {
                "hook-priority"
            } else if verified {
                "hook-verified"
            } else {
                "hook-basic"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{audit_label, Channel};

    #[test]
    fn email_depends_only_on_urgency() {
        assert_eq!(audit_label(Channel::Email, true, false), "email-priority");
        assert_eq!(audit_label(Channel::Email, false, true), "email-standard");
    }

    #[test]
    fn sms_depends_only_on_verification() {
        assert_eq!(audit_label(Channel::Sms, true, true), "sms-direct");
        assert_eq!(audit_label(Channel::Sms, true, false), "sms-pending");
    }

    #[test]
    fn push_requires_both_flags_for_alert() {
        assert_eq!(audit_label(Channel::Push, true, true), "push-alert");
        assert_eq!(audit_label(Channel::Push, true, false), "push-bulk");
        assert_eq!(audit_label(Channel::Push, false, true), "push-bulk");
    }

    #[test]
    fn webhook_prioritizes_verified_before_urgent() {
        assert_eq!(audit_label(Channel::Webhook, true, true), "hook-verified");
        assert_eq!(audit_label(Channel::Webhook, false, true), "hook-verified");
        assert_eq!(audit_label(Channel::Webhook, true, false), "hook-priority");
        assert_eq!(audit_label(Channel::Webhook, false, false), "hook-basic");
    }
}
