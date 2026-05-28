#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Connect,
    Disconnect,
    Message { urgent: bool, has_attachment: bool },
    Retry(u8),
}

pub fn action_for(event: Event) -> &'static str {
    match event {
        Event::Connect => "open_session",
        Event::Disconnect => "close_session",
        Event::Message { urgent, has_attachment: _ } => {
            if urgent {
                "queue"
            } else {
                "deliver"
            }
        }
        Event::Retry(attempt) => {
            if attempt <= 3 {
                "drop"
            } else {
                "backoff"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urgent_message_without_attachment_is_expedited() {
        assert_eq!(action_for(Event::Message { urgent: true, has_attachment: false }), "expedite");
    }

    #[test]
    fn message_with_attachment_is_scanned_even_if_not_urgent() {
        assert_eq!(action_for(Event::Message { urgent: false, has_attachment: true }), "scan");
    }

    #[test]
    fn urgent_message_with_attachment_is_scanned_first() {
        assert_eq!(action_for(Event::Message { urgent: true, has_attachment: true }), "scan");
    }

    #[test]
    fn first_three_retries_back_off() {
        assert_eq!(action_for(Event::Retry(1)), "backoff");
        assert_eq!(action_for(Event::Retry(3)), "backoff");
    }

    #[test]
    fn later_retries_are_dropped() {
        assert_eq!(action_for(Event::Retry(4)), "drop");
    }

    #[test]
    fn connect_and_disconnect_are_direct() {
        assert_eq!(action_for(Event::Connect), "open_session");
        assert_eq!(action_for(Event::Disconnect), "close_session");
    }
}
