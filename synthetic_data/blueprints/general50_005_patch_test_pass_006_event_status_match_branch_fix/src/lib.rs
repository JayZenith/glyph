#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Open,
    Close,
    Retry(u8),
    Fail { code: u16, temporary: bool },
}

pub fn classify(event: Event) -> &'static str {
    match event {
        Event::Open => "active",
        Event::Close => "closed",
        Event::Retry(n) if n < 3 => "retrying",
        Event::Retry(_) => "failed",
        Event::Fail { code, temporary } if temporary || code >= 500 => "retrying",
        Event::Fail { .. } => "closed",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_events_switch_after_third_attempt() {
        assert_eq!(classify(Event::Retry(0)), "retrying");
        assert_eq!(classify(Event::Retry(2)), "retrying");
        assert_eq!(classify(Event::Retry(3)), "failed");
    }

    #[test]
    fn temporary_failures_are_retrying() {
        assert_eq!(
            classify(Event::Fail {
                code: 400,
                temporary: true,
            }),
            "retrying"
        );
    }

    #[test]
    fn server_errors_fail_immediately_when_not_temporary() {
        assert_eq!(
            classify(Event::Fail {
                code: 503,
                temporary: false,
            }),
            "failed"
        );
    }

    #[test]
    fn client_errors_close_when_not_temporary() {
        assert_eq!(
            classify(Event::Fail {
                code: 404,
                temporary: false,
            }),
            "closed"
        );
    }
}
