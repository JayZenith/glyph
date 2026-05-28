#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Notification {
    Welcome,
    SecurityCode,
    PasswordReset,
    BillingAlert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Email,
    Sms,
    Push,
}

pub fn preferred_channel(kind: Notification, verified_phone: bool, push_enabled: bool) -> Channel {
    match kind {
        Notification::Welcome => Channel::Email,
        Notification::SecurityCode | Notification::PasswordReset => {
            if verified_phone {
                Channel::Sms
            } else {
                Channel::Email
            }
        }
        Notification::BillingAlert => {
            if verified_phone {
                Channel::Sms
            } else if push_enabled {
                Channel::Push
            } else {
                Channel::Email
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn welcome_always_uses_email() {
        assert_eq!(preferred_channel(Notification::Welcome, false, false), Channel::Email);
        assert_eq!(preferred_channel(Notification::Welcome, true, true), Channel::Email);
    }

    #[test]
    fn security_code_prefers_sms_when_phone_is_verified() {
        assert_eq!(preferred_channel(Notification::SecurityCode, true, false), Channel::Sms);
        assert_eq!(preferred_channel(Notification::SecurityCode, false, true), Channel::Push);
        assert_eq!(preferred_channel(Notification::SecurityCode, false, false), Channel::Email);
    }

    #[test]
    fn password_reset_prefers_push_before_email_without_phone() {
        assert_eq!(preferred_channel(Notification::PasswordReset, true, true), Channel::Sms);
        assert_eq!(preferred_channel(Notification::PasswordReset, false, true), Channel::Push);
        assert_eq!(preferred_channel(Notification::PasswordReset, false, false), Channel::Email);
    }

    #[test]
    fn billing_alert_keeps_sms_then_push_then_email_priority() {
        assert_eq!(preferred_channel(Notification::BillingAlert, true, true), Channel::Sms);
        assert_eq!(preferred_channel(Notification::BillingAlert, false, true), Channel::Push);
        assert_eq!(preferred_channel(Notification::BillingAlert, false, false), Channel::Email);
    }
}
