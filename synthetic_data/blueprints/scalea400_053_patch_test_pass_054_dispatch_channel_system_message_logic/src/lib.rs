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

pub fn delivery_label(channel: Channel, priority: Priority, is_system: bool) -> &'static str {
    match (channel, priority, is_system) {
        (Channel::Email, Priority::High, _) => "email-expedite",
        (Channel::Email, _, true) => "email-system",
        (Channel::Email, _, false) => "email-standard",
        (Channel::Sms, _, true) => "sms-system",
        (Channel::Sms, Priority::High, false) => "sms-urgent",
        (Channel::Sms, _, false) => "sms-standard",
        (Channel::Push, _, true) => "push-system",
        (Channel::Push, Priority::Low, false) => "push-bulk",
        (Channel::Push, _, false) => "push-normal",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_priority_system_messages_keep_system_label() {
        assert_eq!(delivery_label(Channel::Email, Priority::High, true), "email-system");
        assert_eq!(delivery_label(Channel::Sms, Priority::High, true), "sms-system");
        assert_eq!(delivery_label(Channel::Push, Priority::High, true), "push-system");
    }

    #[test]
    fn non_system_high_priority_uses_expedited_paths() {
        assert_eq!(delivery_label(Channel::Email, Priority::High, false), "email-expedite");
        assert_eq!(delivery_label(Channel::Sms, Priority::High, false), "sms-urgent");
    }

    #[test]
    fn regular_paths_still_work() {
        assert_eq!(delivery_label(Channel::Email, Priority::Normal, false), "email-standard");
        assert_eq!(delivery_label(Channel::Push, Priority::Low, false), "push-bulk");
        assert_eq!(delivery_label(Channel::Push, Priority::Normal, false), "push-normal");
    }
}
