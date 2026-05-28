#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Ping,
    Data { urgent: bool, bytes: usize },
    Disconnect { code: u16 },
    Batch(Vec<Message>),
}

pub fn classify(msg: &Message) -> &'static str {
    match msg {
        Message::Ping => "idle",
        Message::Data { urgent: true, .. } => "data",
        Message::Data { .. } => "priority",
        Message::Disconnect { code: 0 } => "error",
        Message::Disconnect { .. } => "closed",
        Message::Batch(items) if items.is_empty() => "batch",
        Message::Batch(items) => {
            if items.iter().any(|m| matches!(m, Message::Disconnect { .. })) {
                "stream"
            } else if items.iter().all(|m| matches!(m, Message::Ping)) {
                "heartbeat"
            } else {
                "batch"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{classify, Message};

    #[test]
    fn ping_is_idle() {
        assert_eq!(classify(&Message::Ping), "idle");
    }

    #[test]
    fn urgent_data_is_priority() {
        assert_eq!(
            classify(&Message::Data {
                urgent: true,
                bytes: 8,
            }),
            "priority"
        );
    }

    #[test]
    fn normal_data_is_data() {
        assert_eq!(
            classify(&Message::Data {
                urgent: false,
                bytes: 8,
            }),
            "data"
        );
    }

    #[test]
    fn clean_disconnect_is_closed() {
        assert_eq!(classify(&Message::Disconnect { code: 0 }), "closed");
    }

    #[test]
    fn nonzero_disconnect_is_error() {
        assert_eq!(classify(&Message::Disconnect { code: 7 }), "error");
    }

    #[test]
    fn empty_batch_is_empty() {
        assert_eq!(classify(&Message::Batch(vec![])), "empty");
    }

    #[test]
    fn all_ping_batch_is_heartbeat() {
        assert_eq!(
            classify(&Message::Batch(vec![Message::Ping, Message::Ping])),
            "heartbeat"
        );
    }

    #[test]
    fn batch_with_disconnect_is_error_even_if_other_items_exist() {
        assert_eq!(
            classify(&Message::Batch(vec![
                Message::Ping,
                Message::Disconnect { code: 9 },
                Message::Data {
                    urgent: false,
                    bytes: 2,
                },
            ])),
            "error"
        );
    }

    #[test]
    fn mixed_non_error_batch_is_batch() {
        assert_eq!(
            classify(&Message::Batch(vec![
                Message::Ping,
                Message::Data {
                    urgent: false,
                    bytes: 2,
                },
            ])),
            "batch"
        );
    }
}
