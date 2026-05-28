#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Connect { secure: bool },
    Data { compressed: bool, bytes: usize },
    Heartbeat,
    Disconnect { reason: Option<&'static str> },
}

pub fn action(msg: Message) -> &'static str {
    match msg {
        Message::Connect { secure } => {
            if secure { "open-tls" } else { "open" }
        }
        Message::Data { compressed, bytes } => {
            if compressed || bytes == 0 {
                "decompress"
            } else {
                "process"
            }
        }
        Message::Heartbeat => "noop",
        Message::Disconnect { reason } => {
            if reason.is_some() { "close-error" } else { "close" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_actions_depend_on_security() {
        assert_eq!(action(Message::Connect { secure: true }), "open-tls");
        assert_eq!(action(Message::Connect { secure: false }), "open");
    }

    #[test]
    fn data_actions_distinguish_empty_payloads() {
        assert_eq!(action(Message::Data { compressed: true, bytes: 128 }), "decompress");
        assert_eq!(action(Message::Data { compressed: false, bytes: 128 }), "process");
        assert_eq!(action(Message::Data { compressed: false, bytes: 0 }), "ignore");
    }

    #[test]
    fn disconnect_actions_distinguish_error_reasons() {
        assert_eq!(action(Message::Disconnect { reason: Some("timeout") }), "close-error");
        assert_eq!(action(Message::Disconnect { reason: None }), "close");
    }

    #[test]
    fn heartbeat_is_noop() {
        assert_eq!(action(Message::Heartbeat), "noop");
    }
}
