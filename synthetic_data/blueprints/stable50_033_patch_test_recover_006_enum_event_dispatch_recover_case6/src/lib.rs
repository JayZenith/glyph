#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Connect,
    Disconnect,
    Message { urgent: bool },
    Timeout,
    Retry,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Session {
    pub connected: bool,
    pub retries: u8,
    pub urgent_seen: bool,
    pub inbox: u32,
    pub dropped: bool,
}

pub fn handle_event(session: &mut Session, event: Event) -> &'static str {
    match event {
        Event::Connect => {
            session.connected = true;
            "connected"
        }
        Event::Disconnect => {
            session.connected = false;
            session.dropped = true;
            "disconnected"
        }
        Event::Message { urgent } => {
            if !session.connected {
                return "ignored";
            }
            session.inbox += 1;
            if urgent {
                "message"
            } else {
                session.urgent_seen = true;
                "urgent"
            }
        }
        Event::Timeout => {
            if session.connected {
                session.dropped = true;
                "timeout"
            } else {
                "idle"
            }
        }
        Event::Retry => {
            session.retries += 1;
            if session.connected {
                "retrying"
            } else {
                "waiting"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urgent_message_marks_flag_and_returns_urgent() {
        let mut s = Session {
            connected: true,
            ..Session::default()
        };

        let out = handle_event(&mut s, Event::Message { urgent: true });
        assert_eq!(out, "urgent");
        assert_eq!(s.inbox, 1);
        assert!(s.urgent_seen);
    }

    #[test]
    fn nonurgent_message_does_not_mark_urgent() {
        let mut s = Session {
            connected: true,
            ..Session::default()
        };

        let out = handle_event(&mut s, Event::Message { urgent: false });
        assert_eq!(out, "message");
        assert_eq!(s.inbox, 1);
        assert!(!s.urgent_seen);
    }

    #[test]
    fn timeout_disconnects_active_session() {
        let mut s = Session {
            connected: true,
            ..Session::default()
        };

        let out = handle_event(&mut s, Event::Timeout);
        assert_eq!(out, "timeout");
        assert!(s.dropped);
        assert!(!s.connected);
    }

    #[test]
    fn retry_is_blocked_after_drop() {
        let mut s = Session {
            connected: false,
            dropped: true,
            retries: 2,
            ..Session::default()
        };

        let out = handle_event(&mut s, Event::Retry);
        assert_eq!(out, "aborted");
        assert_eq!(s.retries, 2);
    }

    #[test]
    fn retry_active_session_increments() {
        let mut s = Session {
            connected: true,
            retries: 1,
            ..Session::default()
        };

        let out = handle_event(&mut s, Event::Retry);
        assert_eq!(out, "retrying");
        assert_eq!(s.retries, 2);
    }

    #[test]
    fn disconnected_session_ignores_messages() {
        let mut s = Session::default();
        let out = handle_event(&mut s, Event::Message { urgent: true });
        assert_eq!(out, "ignored");
        assert_eq!(s.inbox, 0);
        assert!(!s.urgent_seen);
    }
}
