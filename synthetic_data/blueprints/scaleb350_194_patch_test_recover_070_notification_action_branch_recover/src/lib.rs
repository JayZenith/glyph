#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Email,
    Sms,
    Push,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delivery {
    Draft,
    Queued,
    Sent,
    Failed,
}

pub fn action_for(channel: Channel, state: Delivery) -> &'static str {
    match (channel, state) {
        (_, Delivery::Draft) => "edit",
        (Channel::Email, Delivery::Queued) => "send_email",
        (Channel::Sms, Delivery::Queued) => "send_sms",
        (Channel::Push, Delivery::Queued) => "send_sms",
        (_, Delivery::Sent) => "archive",
        (_, Delivery::Failed) => "retry",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queued_actions_depend_on_channel() {
        assert_eq!(action_for(Channel::Email, Delivery::Queued), "send_email");
        assert_eq!(action_for(Channel::Sms, Delivery::Queued), "send_sms");
        assert_eq!(action_for(Channel::Push, Delivery::Queued), "send_push");
    }

    #[test]
    fn failed_actions_depend_on_channel() {
        assert_eq!(action_for(Channel::Email, Delivery::Failed), "retry_email");
        assert_eq!(action_for(Channel::Sms, Delivery::Failed), "retry_sms");
        assert_eq!(action_for(Channel::Push, Delivery::Failed), "retry_push");
    }

    #[test]
    fn draft_and_sent_are_shared() {
        assert_eq!(action_for(Channel::Push, Delivery::Draft), "edit");
        assert_eq!(action_for(Channel::Email, Delivery::Sent), "archive");
    }
}
