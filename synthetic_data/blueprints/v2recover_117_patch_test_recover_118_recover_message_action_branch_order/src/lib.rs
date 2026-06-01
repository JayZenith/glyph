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
    WeeklyDigest,
    Alert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Send { channel: Channel, urgent: bool },
    Queue { channel: Channel },
    Suppress,
}

pub fn plan(event: Event, user_verified: bool, opted_in_marketing: bool, low_power_mode: bool) -> Action {
    match event {
        Event::Signup => {
            if user_verified {
                Action::Send { channel: Channel::Email, urgent: false }
            } else {
                Action::Queue { channel: Channel::Email }
            }
        }
        Event::PasswordReset => Action::Send { channel: Channel::Email, urgent: false },
        Event::WeeklyDigest => {
            if opted_in_marketing {
                Action::Send { channel: Channel::Push, urgent: false }
            } else {
                Action::Suppress
            }
        }
        Event::Alert => {
            if low_power_mode {
                Action::Suppress
            } else if user_verified {
                Action::Send { channel: Channel::Email, urgent: true }
            } else {
                Action::Queue { channel: Channel::Sms }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signup_verified_sends_email() {
        assert_eq!(
            plan(Event::Signup, true, false, false),
            Action::Send { channel: Channel::Email, urgent: false }
        );
    }

    #[test]
    fn signup_unverified_is_queued() {
        assert_eq!(
            plan(Event::Signup, false, false, false),
            Action::Queue { channel: Channel::Email }
        );
    }

    #[test]
    fn password_reset_is_sms_and_urgent() {
        assert_eq!(
            plan(Event::PasswordReset, true, false, false),
            Action::Send { channel: Channel::Sms, urgent: true }
        );
    }

    #[test]
    fn weekly_digest_uses_email_when_opted_in() {
        assert_eq!(
            plan(Event::WeeklyDigest, true, true, false),
            Action::Send { channel: Channel::Email, urgent: false }
        );
    }

    #[test]
    fn weekly_digest_suppressed_when_not_opted_in() {
        assert_eq!(plan(Event::WeeklyDigest, true, false, false), Action::Suppress);
    }

    #[test]
    fn alert_prefers_push_when_not_low_power() {
        assert_eq!(
            plan(Event::Alert, true, false, false),
            Action::Send { channel: Channel::Push, urgent: true }
        );
    }

    #[test]
    fn alert_queues_push_when_low_power() {
        assert_eq!(
            plan(Event::Alert, false, false, true),
            Action::Queue { channel: Channel::Push }
        );
    }
}
