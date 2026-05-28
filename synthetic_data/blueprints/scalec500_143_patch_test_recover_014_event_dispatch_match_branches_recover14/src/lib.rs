#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Email,
    Sms,
    Push,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Signup,
    PasswordReset,
    Purchase,
    Chargeback,
    Digest,
}

pub fn dispatch(event: Event, channel: Channel, urgent: bool) -> &'static str {
    match event {
        Event::Signup => match channel {
            Channel::Email => "welcome-email",
            Channel::Sms => "welcome-sms",
            Channel::Push => "welcome-push",
        },
        Event::PasswordReset => match channel {
            Channel::Email => "reset-email",
            Channel::Sms => "reset-email",
            Channel::Push => "reset-push",
        },
        Event::Purchase => match channel {
            Channel::Email => "receipt-email",
            Channel::Sms => "receipt-sms",
            Channel::Push => "receipt-push",
        },
        Event::Chargeback => {
            if urgent {
                "chargeback-escalate"
            } else {
                "chargeback-log"
            }
        }
        Event::Digest => {
            if urgent {
                "digest-now"
            } else {
                "digest-batch"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{dispatch, Channel, Event};

    #[test]
    fn signup_routes_to_matching_channel() {
        assert_eq!(dispatch(Event::Signup, Channel::Email, false), "welcome-email");
        assert_eq!(dispatch(Event::Signup, Channel::Sms, false), "welcome-sms");
        assert_eq!(dispatch(Event::Signup, Channel::Push, true), "welcome-push");
    }

    #[test]
    fn password_reset_uses_channel_specific_templates() {
        assert_eq!(dispatch(Event::PasswordReset, Channel::Email, false), "reset-email");
        assert_eq!(dispatch(Event::PasswordReset, Channel::Sms, false), "reset-sms");
        assert_eq!(dispatch(Event::PasswordReset, Channel::Push, true), "reset-push");
    }

    #[test]
    fn purchase_push_is_suppressed_when_not_urgent() {
        assert_eq!(dispatch(Event::Purchase, Channel::Email, false), "receipt-email");
        assert_eq!(dispatch(Event::Purchase, Channel::Sms, false), "receipt-sms");
        assert_eq!(dispatch(Event::Purchase, Channel::Push, false), "skip");
        assert_eq!(dispatch(Event::Purchase, Channel::Push, true), "receipt-push");
    }

    #[test]
    fn chargeback_escalation_depends_on_channel_and_urgency() {
        assert_eq!(dispatch(Event::Chargeback, Channel::Email, false), "chargeback-review");
        assert_eq!(dispatch(Event::Chargeback, Channel::Email, true), "chargeback-escalate");
        assert_eq!(dispatch(Event::Chargeback, Channel::Sms, false), "chargeback-review");
        assert_eq!(dispatch(Event::Chargeback, Channel::Sms, true), "chargeback-escalate");
        assert_eq!(dispatch(Event::Chargeback, Channel::Push, false), "skip");
        assert_eq!(dispatch(Event::Chargeback, Channel::Push, true), "skip");
    }

    #[test]
    fn digest_is_email_only_and_never_urgent() {
        assert_eq!(dispatch(Event::Digest, Channel::Email, false), "digest-batch");
        assert_eq!(dispatch(Event::Digest, Channel::Email, true), "digest-batch");
        assert_eq!(dispatch(Event::Digest, Channel::Sms, false), "skip");
        assert_eq!(dispatch(Event::Digest, Channel::Push, true), "skip");
    }
}
