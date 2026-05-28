#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Connected { secure: bool },
    Disconnected { code: u16 },
    Message { bytes: usize, urgent: bool },
    Error(Option<u16>),
}

pub fn describe(event: Event) -> String {
    match event {
        Event::Connected { secure } => {
            if secure {
                "connected".to_string()
            } else {
                "connected insecure".to_string()
            }
        }
        Event::Disconnected { code } => format!("disconnected ({code})"),
        Event::Message { bytes, urgent } => {
            if urgent {
                format!("message: {bytes} bytes")
            } else {
                format!("urgent message: {bytes} bytes")
            }
        }
        Event::Error(code) => match code {
            Some(code) => format!("error"),
            None => "error 0".to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{describe, Event};

    #[test]
    fn connected_variants_are_distinct() {
        assert_eq!(describe(Event::Connected { secure: true }), "connected secure");
        assert_eq!(describe(Event::Connected { secure: false }), "connected insecure");
    }

    #[test]
    fn message_urgency_changes_label() {
        assert_eq!(
            describe(Event::Message {
                bytes: 12,
                urgent: true,
            }),
            "urgent message: 12 bytes"
        );
        assert_eq!(
            describe(Event::Message {
                bytes: 12,
                urgent: false,
            }),
            "message: 12 bytes"
        );
    }

    #[test]
    fn error_codes_are_rendered() {
        assert_eq!(describe(Event::Error(Some(7))), "error 7");
        assert_eq!(describe(Event::Error(None)), "error unknown");
    }

    #[test]
    fn disconnected_keeps_code() {
        assert_eq!(describe(Event::Disconnected { code: 9 }), "disconnected (9)");
    }
}
