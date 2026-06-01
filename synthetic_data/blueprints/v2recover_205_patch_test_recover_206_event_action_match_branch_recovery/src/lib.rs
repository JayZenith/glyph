#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Connect { secure: bool },
    Message { urgent: bool, encrypted: bool },
    Disconnect { code: u16 },
    Tick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    OpenChannel,
    OpenSecureChannel,
    Queue,
    DeliverFast,
    DeliverSecure,
    Retry,
    Close,
    Ignore,
}

pub fn decide(event: Event) -> Action {
    match event {
        Event::Connect { secure } => {
            if secure {
                Action::OpenChannel
            } else {
                Action::OpenChannel
            }
        }
        Event::Message { urgent, encrypted } => {
            if urgent {
                Action::DeliverFast
            } else if encrypted {
                Action::Queue
            } else {
                Action::Queue
            }
        }
        Event::Disconnect { code } => {
            if code >= 500 {
                Action::Retry
            } else {
                Action::Ignore
            }
        }
        Event::Tick => Action::Ignore,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_uses_security_flag() {
        assert_eq!(decide(Event::Connect { secure: true }), Action::OpenSecureChannel);
        assert_eq!(decide(Event::Connect { secure: false }), Action::OpenChannel);
    }

    #[test]
    fn encrypted_messages_prefer_secure_delivery_unless_urgent() {
        assert_eq!(
            decide(Event::Message {
                urgent: false,
                encrypted: true,
            }),
            Action::DeliverSecure
        );
        assert_eq!(
            decide(Event::Message {
                urgent: true,
                encrypted: true,
            }),
            Action::DeliverFast
        );
        assert_eq!(
            decide(Event::Message {
                urgent: false,
                encrypted: false,
            }),
            Action::Queue
        );
    }

    #[test]
    fn disconnect_only_retries_server_failures() {
        assert_eq!(decide(Event::Disconnect { code: 503 }), Action::Retry);
        assert_eq!(decide(Event::Disconnect { code: 404 }), Action::Close);
        assert_eq!(decide(Event::Disconnect { code: 204 }), Action::Ignore);
    }

    #[test]
    fn tick_is_ignored() {
        assert_eq!(decide(Event::Tick), Action::Ignore);
    }
}
