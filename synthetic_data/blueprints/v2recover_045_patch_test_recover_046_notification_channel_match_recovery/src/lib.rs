#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Low,
    Normal,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Audience {
    User,
    Team,
    Broadcast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Inbox,
    Email,
    Sms,
    Pager,
    Digest,
}

pub fn dispatch_channel(priority: Priority, audience: Audience, quiet_hours: bool) -> Channel {
    match (priority, audience, quiet_hours) {
        (Priority::High, Audience::Broadcast, _) => Channel::Pager,
        (Priority::High, _, false) => Channel::Email,
        (_, Audience::Broadcast, _) => Channel::Digest,
        (Priority::Low, _, true) => Channel::Inbox,
        (_, Audience::Team, true) => Channel::Email,
        _ => Channel::Inbox,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_priority_user_escalates_based_on_quiet_hours() {
        assert_eq!(dispatch_channel(Priority::High, Audience::User, false), Channel::Sms);
        assert_eq!(dispatch_channel(Priority::High, Audience::User, true), Channel::Pager);
    }

    #[test]
    fn team_notifications_have_special_routing() {
        assert_eq!(dispatch_channel(Priority::Normal, Audience::Team, true), Channel::Digest);
        assert_eq!(dispatch_channel(Priority::High, Audience::Team, false), Channel::Pager);
    }

    #[test]
    fn broadcasts_and_low_priority_follow_fallback_rules() {
        assert_eq!(dispatch_channel(Priority::Low, Audience::Broadcast, false), Channel::Digest);
        assert_eq!(dispatch_channel(Priority::Low, Audience::User, true), Channel::Digest);
        assert_eq!(dispatch_channel(Priority::Normal, Audience::User, false), Channel::Inbox);
    }
}
