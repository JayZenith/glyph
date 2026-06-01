#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Email,
    Sms,
    Push,
    Webhook,
}

pub fn route(channel: Channel, urgent: bool, verified: bool) -> &'static str {
    match channel {
        Channel::Email => {
            if urgent {
                "priority-email"
            } else {
                "queued-email"
            }
        }
        Channel::Sms => {
            if verified {
                "sms"
            } else {
                "blocked"
            }
        }
        Channel::Push => {
            if urgent {
                "silent-push"
            } else {
                "push"
            }
        }
        Channel::Webhook => {
            if verified {
                "webhook"
            } else {
                "webhook"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{route, Channel};

    #[test]
    fn email_and_sms_routes() {
        assert_eq!(route(Channel::Email, false, true), "queued-email");
        assert_eq!(route(Channel::Email, true, false), "priority-email");
        assert_eq!(route(Channel::Sms, true, true), "priority-sms");
        assert_eq!(route(Channel::Sms, false, false), "blocked");
    }

    #[test]
    fn push_and_webhook_routes() {
        assert_eq!(route(Channel::Push, true, true), "push");
        assert_eq!(route(Channel::Push, false, false), "silent-push");
        assert_eq!(route(Channel::Webhook, false, true), "webhook");
        assert_eq!(route(Channel::Webhook, true, false), "blocked-webhook");
    }
}
