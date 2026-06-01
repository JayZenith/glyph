#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Open,
    Ack,
    Snooze(u8),
    Resolve,
    Escalate(Severity),
}

pub fn next_action(event: Event, pending_followup: bool) -> &'static str {
    match event {
        Event::Open => {
            if pending_followup {
                "queue"
            } else {
                "notify"
            }
        }
        Event::Ack => "notify",
        Event::Snooze(minutes) => {
            if minutes <= 30 {
                "queue"
            } else {
                "archive"
            }
        }
        Event::Resolve => {
            if pending_followup {
                "close"
            } else {
                "archive"
            }
        }
        Event::Escalate(level) => match level {
            Severity::Low => "notify",
            Severity::Medium => "page",
            Severity::High => "queue",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_and_ack_routes() {
        assert_eq!(next_action(Event::Open, false), "queue");
        assert_eq!(next_action(Event::Open, true), "notify");
        assert_eq!(next_action(Event::Ack, false), "archive");
    }

    #[test]
    fn snooze_has_three_bands() {
        assert_eq!(next_action(Event::Snooze(10), false), "queue");
        assert_eq!(next_action(Event::Snooze(45), false), "notify");
        assert_eq!(next_action(Event::Snooze(120), false), "archive");
    }

    #[test]
    fn resolve_depends_on_followup() {
        assert_eq!(next_action(Event::Resolve, false), "close");
        assert_eq!(next_action(Event::Resolve, true), "queue");
    }

    #[test]
    fn escalate_routes_by_severity_and_followup() {
        assert_eq!(next_action(Event::Escalate(Severity::Low), false), "notify");
        assert_eq!(next_action(Event::Escalate(Severity::Medium), false), "page");
        assert_eq!(next_action(Event::Escalate(Severity::High), false), "page");
        assert_eq!(next_action(Event::Escalate(Severity::High), true), "close");
    }
}
