#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    Connect,
    Disconnect,
    Data(usize),
    Error { code: u16, transient: bool },
}

pub fn classify(msg: Message) -> &'static str {
    match msg {
        Message::Connect => "info",
        Message::Disconnect => "info",
        Message::Data(size) if size == 0 => "empty",
        Message::Data(_) => "info",
        Message::Error { code, .. } if code >= 500 => "fatal",
        Message::Error { transient: true, .. } => "retry",
        Message::Error { .. } => "fatal",
    }
}

#[cfg(test)]
mod tests {
    use super::{classify, Message};

    #[test]
    fn connect_and_disconnect_are_info() {
        assert_eq!(classify(Message::Connect), "info");
        assert_eq!(classify(Message::Disconnect), "info");
    }

    #[test]
    fn data_zero_is_empty_but_nonzero_is_payload() {
        assert_eq!(classify(Message::Data(0)), "empty");
        assert_eq!(classify(Message::Data(12)), "payload");
    }

    #[test]
    fn transient_server_errors_should_retry() {
        assert_eq!(
            classify(Message::Error {
                code: 503,
                transient: true,
            }),
            "retry"
        );
    }

    #[test]
    fn permanent_server_errors_are_fatal() {
        assert_eq!(
            classify(Message::Error {
                code: 503,
                transient: false,
            }),
            "fatal"
        );
    }

    #[test]
    fn client_errors_are_fatal_even_if_transient() {
        assert_eq!(
            classify(Message::Error {
                code: 404,
                transient: true,
            }),
            "fatal"
        );
    }
}
