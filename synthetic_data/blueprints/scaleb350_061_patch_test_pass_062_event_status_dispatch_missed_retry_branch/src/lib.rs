#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Connected,
    Disconnected { retryable: bool },
    Data { valid: bool },
    Timeout { attempt: u8 },
}

pub fn classify(event: Event) -> &'static str {
    match event {
        Event::Connected => "ready",
        Event::Disconnected { retryable } => {
            if retryable { "offline" } else { "dead" }
        }
        Event::Data { valid } => {
            if valid { "accepted" } else { "rejected" }
        }
        Event::Timeout { attempt } => {
            if attempt == 0 { "retry" } else { "dead" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{classify, Event};

    #[test]
    fn connected_is_ready() {
        assert_eq!(classify(Event::Connected), "ready");
    }

    #[test]
    fn disconnected_branch_distinguishes_retryable() {
        assert_eq!(classify(Event::Disconnected { retryable: true }), "offline");
        assert_eq!(classify(Event::Disconnected { retryable: false }), "dead");
    }

    #[test]
    fn data_branch_distinguishes_validity() {
        assert_eq!(classify(Event::Data { valid: true }), "accepted");
        assert_eq!(classify(Event::Data { valid: false }), "rejected");
    }

    #[test]
    fn timeout_allows_two_retries_before_dead() {
        assert_eq!(classify(Event::Timeout { attempt: 0 }), "retry");
        assert_eq!(classify(Event::Timeout { attempt: 1 }), "retry");
        assert_eq!(classify(Event::Timeout { attempt: 2 }), "dead");
        assert_eq!(classify(Event::Timeout { attempt: 5 }), "dead");
    }
}
