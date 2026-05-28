#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Email,
    Sms,
    Push,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Low,
    Normal,
    High,
}

pub fn route(channel: Channel, priority: Priority, muted: bool) -> &'static str {
    match (channel, priority, muted) {
        (Channel::Email, Priority::High, _) => "email-express",
        (Channel::Email, _, true) => "hold",
        (Channel::Email, _, false) => "email-batch",
        (Channel::Sms, Priority::High, false) => "sms-now",
        (Channel::Sms, _, true) => "hold",
        (Channel::Sms, _, false) => "sms-queue",
        (Channel::Push, _, true) => "hold",
        (Channel::Push, Priority::Low, false) => "push-bulk",
        (Channel::Push, _, false) => "push-now",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_high_is_held_when_muted() {
        assert_eq!(route(Channel::Email, Priority::High, true), "hold");
    }

    #[test]
    fn email_high_not_muted_is_express() {
        assert_eq!(route(Channel::Email, Priority::High, false), "email-express");
    }

    #[test]
    fn sms_high_is_immediate_even_if_muted() {
        assert_eq!(route(Channel::Sms, Priority::High, true), "sms-now");
    }

    #[test]
    fn push_low_not_muted_is_bulk() {
        assert_eq!(route(Channel::Push, Priority::Low, false), "push-bulk");
    }

    #[test]
    fn push_normal_muted_is_held() {
        assert_eq!(route(Channel::Push, Priority::Normal, true), "hold");
    }
}
