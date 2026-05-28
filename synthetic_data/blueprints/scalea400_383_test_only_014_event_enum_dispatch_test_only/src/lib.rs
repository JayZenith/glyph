#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Connect { secure: bool },
    Message { urgent: bool, body: String },
    Disconnect { code: u16 },
    Tick,
}

pub fn classify(event: &Event) -> &'static str {
    match event {
        Event::Connect { secure: true } => "connect-secure",
        Event::Connect { secure: false } => "connect-plain",
        Event::Message { urgent: true, .. } => "message-priority",
        Event::Message { urgent: false, body } if body.is_empty() => "message-empty",
        Event::Message { urgent: false, .. } => "message-normal",
        Event::Disconnect { code: 0 } => "disconnect-clean",
        Event::Disconnect { code } if *code >= 4000 => "disconnect-fatal",
        Event::Disconnect { .. } => "disconnect-recoverable",
        Event::Tick => "tick",
    }
}

pub fn should_log(event: &Event) -> bool {
    match event {
        Event::Tick => false,
        Event::Connect { .. } => true,
        Event::Message { urgent, body } => *urgent || !body.is_empty(),
        Event::Disconnect { code } => *code != 0,
    }
}

pub fn route(event: Event) -> (&'static str, bool) {
    match event {
        Event::Connect { secure } => (if secure { "handshake" } else { "session" }, true),
        Event::Message { urgent, body } => {
            let lane = if urgent {
                "alerts"
            } else if body.is_empty() {
                "dropbox"
            } else {
                "inbox"
            };
            (lane, true)
        }
        Event::Disconnect { code } => (if code >= 4000 { "ops" } else { "session" }, false),
        Event::Tick => ("scheduler", false),
    }
}

#[cfg(test)]
mod tests {
    use super::{classify, route, should_log, Event};

    #[test]
    fn classify_covers_message_branches() {
        assert_eq!(classify(&Event::Message { urgent: true, body: "hi".into() }), "message-priority");
        assert_eq!(classify(&Event::Message { urgent: false, body: String::new() }), "message-empty");
        assert_eq!(classify(&Event::Message { urgent: false, body: "note".into() }), "message-normal");
    }

    #[test]
    fn classify_covers_disconnect_severity() {
        assert_eq!(classify(&Event::Disconnect { code: 0 }), "disconnect-clean");
        assert_eq!(classify(&Event::Disconnect { code: 42 }), "disconnect-recoverable");
        assert_eq!(classify(&Event::Disconnect { code: 4001 }), "disconnect-fatal");
    }

    #[test]
    fn should_log_respects_variant_rules() {
        assert!(!should_log(&Event::Tick));
        assert!(should_log(&Event::Connect { secure: false }));
        assert!(should_log(&Event::Message { urgent: true, body: String::new() }));
        assert!(!should_log(&Event::Message { urgent: false, body: String::new() }));
        assert!(should_log(&Event::Message { urgent: false, body: "payload".into() }));
        assert!(!should_log(&Event::Disconnect { code: 0 }));
        assert!(should_log(&Event::Disconnect { code: 9 }));
    }

    #[test]
    fn route_dispatches_expected_lane_and_activity() {
        assert_eq!(route(Event::Connect { secure: true }), ("handshake", true));
        assert_eq!(route(Event::Connect { secure: false }), ("session", true));
        assert_eq!(route(Event::Message { urgent: true, body: "warn".into() }), ("alerts", true));
        assert_eq!(route(Event::Message { urgent: false, body: String::new() }), ("dropbox", true));
        assert_eq!(route(Event::Message { urgent: false, body: "memo".into() }), ("inbox", true));
        assert_eq!(route(Event::Disconnect { code: 17 }), ("session", false));
        assert_eq!(route(Event::Disconnect { code: 5000 }), ("ops", false));
        assert_eq!(route(Event::Tick), ("scheduler", false));
    }
}
