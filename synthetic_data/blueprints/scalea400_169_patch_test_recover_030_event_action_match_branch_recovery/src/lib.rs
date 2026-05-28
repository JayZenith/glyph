#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Connected { secure: bool },
    Message { urgent: bool, encrypted: bool },
    Closed { code: u16 },
    Retry { attempt: u8, after_error: bool },
}

pub fn plan(event: Event) -> Vec<&'static str> {
    let mut actions = Vec::new();
    match event {
        Event::Connected { secure } => {
            actions.push("open");
            if !secure {
                actions.push("handshake");
            }
        }
        Event::Message { urgent, encrypted } => {
            actions.push("store");
            if urgent {
                actions.push("queue");
            }
            if encrypted {
                actions.push("scan");
            }
        }
        Event::Closed { code } => {
            actions.push("close");
            if code >= 500 {
                actions.push("archive");
            }
        }
        Event::Retry {
            attempt,
            after_error,
        } => {
            if attempt > 3 {
                actions.push("retry");
                actions.push("backoff");
            } else if after_error {
                actions.push("retry");
                actions.push("notify");
            } else {
                actions.push("retry");
            }
        }
    }
    actions
}

#[cfg(test)]
mod tests {
    use super::{plan, Event};

    #[test]
    fn secure_connection_skips_handshake() {
        assert_eq!(plan(Event::Connected { secure: true }), vec!["open"]);
    }

    #[test]
    fn insecure_connection_requires_handshake() {
        assert_eq!(
            plan(Event::Connected { secure: false }),
            vec!["open", "handshake"]
        );
    }

    #[test]
    fn urgent_encrypted_message_is_prioritized_and_scanned() {
        assert_eq!(
            plan(Event::Message {
                urgent: true,
                encrypted: true,
            }),
            vec!["store", "prioritize", "decrypt"]
        );
    }

    #[test]
    fn plain_message_is_only_stored() {
        assert_eq!(
            plan(Event::Message {
                urgent: false,
                encrypted: false,
            }),
            vec!["store"]
        );
    }

    #[test]
    fn server_error_close_records_alert() {
        assert_eq!(
            plan(Event::Closed { code: 503 }),
            vec!["close", "alert"]
        );
    }

    #[test]
    fn normal_close_is_just_close() {
        assert_eq!(plan(Event::Closed { code: 200 }), vec!["close"]);
    }

    #[test]
    fn retry_after_error_notifies_and_retries() {
        assert_eq!(
            plan(Event::Retry {
                attempt: 2,
                after_error: true,
            }),
            vec!["retry", "notify"]
        );
    }

    #[test]
    fn high_attempt_retry_without_error_backs_off() {
        assert_eq!(
            plan(Event::Retry {
                attempt: 4,
                after_error: false,
            }),
            vec!["retry", "backoff"]
        );
    }

    #[test]
    fn high_attempt_after_error_includes_notify_before_backoff() {
        assert_eq!(
            plan(Event::Retry {
                attempt: 5,
                after_error: true,
            }),
            vec!["retry", "notify", "backoff"]
        );
    }
}
