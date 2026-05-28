#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Email,
    Sms,
    Push,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Welcome,
    PasswordReset,
    Receipt,
}

pub fn template(event: Event, channel: Channel, urgent: bool) -> &'static str {
    match event {
        Event::Welcome => match channel {
            Channel::Email => "welcome-email",
            Channel::Sms => "welcome-sms",
            Channel::Push => "welcome-push",
        },
        Event::PasswordReset => match channel {
            Channel::Email => {
                if urgent {
                    "reset-email"
                } else {
                    "reset-email"
                }
            }
            Channel::Sms => "reset-sms",
            Channel::Push => "reset-push",
        },
        Event::Receipt => {
            if urgent {
                "receipt-priority"
            } else {
                "receipt-email"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn welcome_uses_channel_specific_templates() {
        assert_eq!(template(Event::Welcome, Channel::Email, false), "welcome-email");
        assert_eq!(template(Event::Welcome, Channel::Sms, false), "welcome-sms");
        assert_eq!(template(Event::Welcome, Channel::Push, true), "welcome-push");
    }

    #[test]
    fn password_reset_email_changes_when_urgent() {
        assert_eq!(template(Event::PasswordReset, Channel::Email, false), "reset-email");
        assert_eq!(template(Event::PasswordReset, Channel::Email, true), "reset-email-urgent");
    }

    #[test]
    fn receipt_ignores_urgency_and_still_uses_channel() {
        assert_eq!(template(Event::Receipt, Channel::Email, false), "receipt-email");
        assert_eq!(template(Event::Receipt, Channel::Sms, true), "receipt-sms");
        assert_eq!(template(Event::Receipt, Channel::Push, true), "receipt-push");
    }
}
