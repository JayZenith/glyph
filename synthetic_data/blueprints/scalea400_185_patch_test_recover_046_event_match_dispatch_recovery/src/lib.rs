#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Connect,
    Disconnect,
    Message { urgent: bool, body: &'static str },
    Tick(u32),
    Error { code: u16, fatal: bool },
}

pub fn classify(event: Event) -> &'static str {
    match event {
        Event::Connect => "session",
        Event::Disconnect => "session",
        Event::Message { urgent, .. } => {
            if urgent {
                "alert"
            } else {
                "session"
            }
        }
        Event::Tick(n) => {
            if n == 0 {
                "ignore"
            } else {
                "session"
            }
        }
        Event::Error { fatal, .. } => {
            if fatal {
                "fatal"
            } else {
                "session"
            }
        }
    }
}

pub fn route(event: Event) -> &'static str {
    match event {
        Event::Connect => "audit",
        Event::Disconnect => "audit",
        Event::Message { urgent: true, .. } => "queue:urgent",
        Event::Message { urgent: false, .. } => "queue:normal",
        Event::Tick(_) => "metrics",
        Event::Error { fatal: true, .. } => "ops",
        Event::Error { fatal: false, .. } => "audit",
    }
}

pub fn should_retry(event: Event) -> bool {
    match event {
        Event::Error { code, fatal } => !fatal && code >= 500,
        Event::Disconnect => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{classify, route, should_retry, Event};

    #[test]
    fn urgent_and_empty_messages_are_distinguished() {
        assert_eq!(classify(Event::Message { urgent: true, body: "ping" }), "alert");
        assert_eq!(classify(Event::Message { urgent: false, body: "" }), "ignore");
        assert_eq!(route(Event::Message { urgent: false, body: "" }), "drop");
    }

    #[test]
    fn tick_zero_is_ignored_but_other_ticks_are_internal() {
        assert_eq!(classify(Event::Tick(0)), "ignore");
        assert_eq!(classify(Event::Tick(3)), "internal");
        assert_eq!(route(Event::Tick(0)), "metrics");
    }

    #[test]
    fn nonfatal_errors_depend_on_code() {
        assert_eq!(classify(Event::Error { code: 404, fatal: false }), "warning");
        assert_eq!(classify(Event::Error { code: 503, fatal: false }), "transient");
        assert_eq!(classify(Event::Error { code: 2, fatal: true }), "fatal");
    }

    #[test]
    fn retry_policy_only_for_server_side_nonfatal_errors() {
        assert!(!should_retry(Event::Error { code: 404, fatal: false }));
        assert!(should_retry(Event::Error { code: 503, fatal: false }));
        assert!(!should_retry(Event::Disconnect));
    }
}
