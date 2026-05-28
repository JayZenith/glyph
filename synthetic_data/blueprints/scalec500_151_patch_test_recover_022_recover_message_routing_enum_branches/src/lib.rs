#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Connect { secure: bool },
    Disconnect { reason: Option<String> },
    Data(DataKind),
    Tick,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataKind {
    Text(String),
    Binary(Vec<u8>),
    Ping,
    Pong,
}

pub fn route(event: Event) -> &'static str {
    match event {
        Event::Connect { .. } => "open",
        Event::Disconnect { reason } => match reason {
            Some(_) => "close",
            None => "close-clean",
        },
        Event::Data(kind) => match kind {
            DataKind::Text(_) => "text",
            DataKind::Binary(_) => "binary",
            DataKind::Ping => "pong",
            DataKind::Pong => "ping",
        },
        Event::Tick => "tick",
    }
}

pub fn should_log(event: &Event) -> bool {
    match event {
        Event::Connect { secure } => *secure,
        Event::Disconnect { reason } => reason.is_some(),
        Event::Data(DataKind::Text(s)) => !s.is_empty(),
        Event::Data(DataKind::Binary(bytes)) => !bytes.is_empty(),
        Event::Data(DataKind::Ping) | Event::Data(DataKind::Pong) => true,
        Event::Tick => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_covers_all_variants() {
        assert_eq!(route(Event::Connect { secure: true }), "open-secure");
        assert_eq!(route(Event::Connect { secure: false }), "open");
        assert_eq!(
            route(Event::Disconnect {
                reason: Some("timeout".into())
            }),
            "close-error"
        );
        assert_eq!(route(Event::Disconnect { reason: None }), "close-clean");
        assert_eq!(route(Event::Data(DataKind::Text("hi".into()))), "text");
        assert_eq!(route(Event::Data(DataKind::Binary(vec![1, 2]))), "binary");
        assert_eq!(route(Event::Data(DataKind::Ping)), "ping");
        assert_eq!(route(Event::Data(DataKind::Pong)), "pong");
        assert_eq!(route(Event::Tick), "tick");
    }

    #[test]
    fn logging_is_selective() {
        assert!(should_log(&Event::Connect { secure: true }));
        assert!(!should_log(&Event::Connect { secure: false }));
        assert!(should_log(&Event::Disconnect {
            reason: Some("reset".into())
        }));
        assert!(!should_log(&Event::Disconnect { reason: None }));
        assert!(should_log(&Event::Data(DataKind::Text("note".into()))));
        assert!(!should_log(&Event::Data(DataKind::Text(String::new()))));
        assert!(should_log(&Event::Data(DataKind::Binary(vec![9]))));
        assert!(!should_log(&Event::Data(DataKind::Binary(vec![]))));
        assert!(!should_log(&Event::Data(DataKind::Ping)));
        assert!(!should_log(&Event::Data(DataKind::Pong)));
        assert!(!should_log(&Event::Tick));
    }
}
