#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Welcome,
    PasswordReset,
    BillingAlert,
    SecurityNotice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserTier {
    Free,
    Pro,
    Enterprise,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    InApp,
    Email,
    Sms,
    PhoneCall,
}

pub fn preferred_channel(event: Event, tier: UserTier, urgent: bool) -> Channel {
    match event {
        Event::Welcome => match tier {
            UserTier::Free => Channel::InApp,
            UserTier::Pro | UserTier::Enterprise => Channel::Email,
        },
        Event::PasswordReset => {
            if urgent {
                Channel::Sms
            } else {
                Channel::Email
            }
        }
        Event::BillingAlert => match (tier, urgent) {
            (UserTier::Enterprise, true) => Channel::PhoneCall,
            (_, true) => Channel::Sms,
            _ => Channel::Email,
        },
        Event::SecurityNotice => match tier {
            UserTier::Enterprise => Channel::PhoneCall,
            _ => Channel::Email,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn welcome_uses_in_app_for_free_users() {
        assert_eq!(preferred_channel(Event::Welcome, UserTier::Free, false), Channel::InApp);
    }

    #[test]
    fn password_reset_urgent_uses_sms() {
        assert_eq!(preferred_channel(Event::PasswordReset, UserTier::Pro, true), Channel::Sms);
    }

    #[test]
    fn billing_alert_urgent_enterprise_uses_phone_call() {
        assert_eq!(preferred_channel(Event::BillingAlert, UserTier::Enterprise, true), Channel::PhoneCall);
    }

    #[test]
    fn security_notice_urgent_enterprise_uses_sms_not_phone_call() {
        assert_eq!(preferred_channel(Event::SecurityNotice, UserTier::Enterprise, true), Channel::Sms);
    }

    #[test]
    fn security_notice_non_urgent_enterprise_uses_email() {
        assert_eq!(preferred_channel(Event::SecurityNotice, UserTier::Enterprise, false), Channel::Email);
    }

    #[test]
    fn security_notice_non_urgent_free_uses_email() {
        assert_eq!(preferred_channel(Event::SecurityNotice, UserTier::Free, false), Channel::Email);
    }
}
