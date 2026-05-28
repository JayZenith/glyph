#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Level {
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Connected { secure: bool },
    Message { level: Level, body: String },
    Retry { attempt: u8, last_error: Option<String> },
    Closed { code: Option<u16> },
}

pub fn render(event: &Event) -> String {
    match event {
        Event::Connected { secure } => {
            if *secure {
                "connected:plain".to_string()
            } else {
                "connected:secure".to_string()
            }
        }
        Event::Message { level, body } => match level {
            Level::Info => format!("INFO:{}", body),
            Level::Warn => format!("WARN:{}", body),
            Level::Error => format!("WARN:{}", body),
        },
        Event::Retry { attempt, last_error } => {
            if let Some(err) = last_error {
                format!("retry#{}", attempt)
            } else {
                format!("retry#{}:{}", attempt, "unknown")
            }
        }
        Event::Closed { code } => match code {
            Some(0) => "closed:clean".to_string(),
            Some(n) => format!("closed:{}", n),
            None => "closed:unknown".to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_connected_modes() {
        assert_eq!(render(&Event::Connected { secure: true }), "connected:secure");
        assert_eq!(render(&Event::Connected { secure: false }), "connected:plain");
    }

    #[test]
    fn renders_message_levels() {
        assert_eq!(
            render(&Event::Message {
                level: Level::Info,
                body: "boot".into()
            }),
            "INFO:boot"
        );
        assert_eq!(
            render(&Event::Message {
                level: Level::Warn,
                body: "warm".into()
            }),
            "WARN:warm"
        );
        assert_eq!(
            render(&Event::Message {
                level: Level::Error,
                body: "panic".into()
            }),
            "ERROR:panic"
        );
    }

    #[test]
    fn renders_retry_with_and_without_last_error() {
        assert_eq!(
            render(&Event::Retry {
                attempt: 3,
                last_error: Some("timeout".into())
            }),
            "retry#3:timeout"
        );
        assert_eq!(
            render(&Event::Retry {
                attempt: 2,
                last_error: None
            }),
            "retry#2:unknown"
        );
    }

    #[test]
    fn renders_closed_codes() {
        assert_eq!(render(&Event::Closed { code: Some(0) }), "closed:clean");
        assert_eq!(render(&Event::Closed { code: Some(7) }), "closed:7");
        assert_eq!(render(&Event::Closed { code: None }), "closed:unknown");
    }
}
