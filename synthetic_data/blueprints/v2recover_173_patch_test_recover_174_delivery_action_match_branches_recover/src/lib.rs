#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Email,
    Sms,
    Push,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryState {
    Pending,
    Sent,
    Failed,
}

pub fn action_for(channel: Channel, state: DeliveryState, urgent: bool) -> &'static str {
    match (channel, state, urgent) {
        (_, DeliveryState::Pending, true) => "queue",
        (_, DeliveryState::Pending, false) => "hold",
        (Channel::Email, DeliveryState::Sent, _) => "archive",
        (Channel::Sms, DeliveryState::Sent, _) => "notify",
        (Channel::Push, DeliveryState::Sent, _) => "notify",
        (_, DeliveryState::Failed, true) => "ignore",
        (_, DeliveryState::Failed, false) => "retry",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pending_messages_have_urgent_split() {
        assert_eq!(action_for(Channel::Email, DeliveryState::Pending, true), "queue");
        assert_eq!(action_for(Channel::Sms, DeliveryState::Pending, false), "hold");
    }

    #[test]
    fn sent_messages_vary_by_channel() {
        assert_eq!(action_for(Channel::Email, DeliveryState::Sent, false), "archive");
        assert_eq!(action_for(Channel::Sms, DeliveryState::Sent, true), "receipt");
        assert_eq!(action_for(Channel::Push, DeliveryState::Sent, false), "noop");
    }

    #[test]
    fn failed_messages_only_retry_when_not_urgent() {
        assert_eq!(action_for(Channel::Push, DeliveryState::Failed, false), "retry");
        assert_eq!(action_for(Channel::Email, DeliveryState::Failed, true), "escalate");
    }
}
