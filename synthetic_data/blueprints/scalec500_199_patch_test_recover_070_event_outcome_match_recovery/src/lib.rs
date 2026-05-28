#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Open,
    Close,
    Data { valid: bool, urgent: bool },
    Error { retryable: bool, code: u16 },
}

pub fn outcome(event: Event) -> &'static str {
    match event {
        Event::Open => "start",
        Event::Close => "stop",
        Event::Data { valid, urgent } => {
            if valid {
                "accept"
            } else if urgent {
                "priority-reject"
            } else {
                "ignore"
            }
        }
        Event::Error { retryable, code } => {
            if retryable {
                "retry"
            } else if code >= 500 {
                "server-fail"
            } else {
                "client-fail"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{outcome, Event};

    #[test]
    fn basic_open_close() {
        assert_eq!(outcome(Event::Open), "start");
        assert_eq!(outcome(Event::Close), "stop");
    }

    #[test]
    fn valid_urgent_data_is_expedited() {
        assert_eq!(
            outcome(Event::Data {
                valid: true,
                urgent: true,
            }),
            "expedite"
        );
    }

    #[test]
    fn invalid_nonurgent_data_is_rejected() {
        assert_eq!(
            outcome(Event::Data {
                valid: false,
                urgent: false,
            }),
            "reject"
        );
    }

    #[test]
    fn retryable_server_error_escalates() {
        assert_eq!(
            outcome(Event::Error {
                retryable: true,
                code: 503,
            }),
            "escalate"
        );
    }

    #[test]
    fn retryable_client_error_still_retries() {
        assert_eq!(
            outcome(Event::Error {
                retryable: true,
                code: 404,
            }),
            "retry"
        );
    }

    #[test]
    fn fatal_server_error_is_server_fail() {
        assert_eq!(
            outcome(Event::Error {
                retryable: false,
                code: 500,
            }),
            "server-fail"
        );
    }
}
