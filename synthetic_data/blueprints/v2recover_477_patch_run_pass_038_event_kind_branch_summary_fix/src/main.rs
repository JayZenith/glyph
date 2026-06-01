enum Event {
    Connected,
    Disconnected { retries: u8 },
    Error { code: u16, retryable: bool },
}

fn summarize(event: &Event) -> String {
    match event {
        Event::Connected => "ok:connected".to_string(),
        Event::Disconnected { retries } => format!("warn:retry in {}s", retries),
        Event::Error { code, retryable } => {
            if *retryable {
                format!("err:{}", code)
            } else {
                format!("err:timeout after {} retries", code)
            }
        }
    }
}

fn main() {
    let events = [
        Event::Connected,
        Event::Disconnected { retries: 30 },
        Event::Error {
            code: 42,
            retryable: true,
        },
        Event::Disconnected { retries: 3 },
        Event::Error {
            code: 2,
            retryable: false,
        },
    ];

    let out = events.iter().map(summarize).collect::<Vec<_>>().join("|");
    println!("{}", out);
}
