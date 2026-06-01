#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Connect { secure: bool, resumed: bool },
    Message { encrypted: bool, urgent: bool },
    Disconnect { clean: bool },
}

pub fn next_action(event: Event) -> &'static str {
    match event {
        Event::Connect { secure, resumed } => {
            if secure {
                "send_welcome_back"
            } else if resumed {
                "resume_session"
            } else {
                "reject_insecure"
            }
        }
        Event::Message { encrypted, urgent } => {
            if urgent {
                "queue_normal"
            } else if encrypted {
                "queue_secure"
            } else {
                "drop_unencrypted"
            }
        }
        Event::Disconnect { clean } => {
            if clean {
                "alert_disconnect"
            } else {
                "close_session"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{next_action, Event};

    #[test]
    fn connect_logic_prefers_resumed_and_rejects_plain_new_sessions() {
        assert_eq!(
            next_action(Event::Connect {
                secure: true,
                resumed: true,
            }),
            "resume_session"
        );
        assert_eq!(
            next_action(Event::Connect {
                secure: true,
                resumed: false,
            }),
            "send_welcome"
        );
        assert_eq!(
            next_action(Event::Connect {
                secure: false,
                resumed: false,
            }),
            "reject_insecure"
        );
    }

    #[test]
    fn message_logic_routes_urgent_secure_and_plain_cases() {
        assert_eq!(
            next_action(Event::Message {
                encrypted: true,
                urgent: true,
            }),
            "expedite_secure"
        );
        assert_eq!(
            next_action(Event::Message {
                encrypted: true,
                urgent: false,
            }),
            "queue_secure"
        );
        assert_eq!(
            next_action(Event::Message {
                encrypted: false,
                urgent: true,
            }),
            "drop_unencrypted"
        );
    }

    #[test]
    fn disconnect_logic_alerts_only_on_unclean_drop() {
        assert_eq!(
            next_action(Event::Disconnect { clean: true }),
            "close_session"
        );
        assert_eq!(
            next_action(Event::Disconnect { clean: false }),
            "alert_disconnect"
        );
    }
}
