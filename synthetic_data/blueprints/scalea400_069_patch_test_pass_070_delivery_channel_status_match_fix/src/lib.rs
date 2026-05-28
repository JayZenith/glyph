#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Email,
    Sms,
    Push,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryState {
    Queued,
    Sent,
    Failed { retryable: bool },
    Bounced,
}

pub fn display_status(channel: Channel, state: DeliveryState) -> &'static str {
    match state {
        DeliveryState::Queued => "pending",
        DeliveryState::Sent => "delivered",
        DeliveryState::Failed { retryable } => {
            if retryable {
                "retrying"
            } else {
                "failed"
            }
        }
        DeliveryState::Bounced => match channel {
            Channel::Email => "failed",
            Channel::Sms => "bounced",
            Channel::Push => "bounced",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queued_is_pending_for_all_channels() {
        assert_eq!(display_status(Channel::Email, DeliveryState::Queued), "pending");
        assert_eq!(display_status(Channel::Sms, DeliveryState::Queued), "pending");
        assert_eq!(display_status(Channel::Push, DeliveryState::Queued), "pending");
    }

    #[test]
    fn failed_state_distinguishes_retryable() {
        assert_eq!(
            display_status(Channel::Sms, DeliveryState::Failed { retryable: true }),
            "retrying"
        );
        assert_eq!(
            display_status(Channel::Push, DeliveryState::Failed { retryable: false }),
            "failed"
        );
    }

    #[test]
    fn bounced_depends_on_channel_capability() {
        assert_eq!(display_status(Channel::Email, DeliveryState::Bounced), "bounced");
        assert_eq!(display_status(Channel::Sms, DeliveryState::Bounced), "undeliverable");
        assert_eq!(display_status(Channel::Push, DeliveryState::Bounced), "failed");
    }
}
