enum Channel {
    Email,
    Sms,
    Push,
}

enum Event {
    Send {
        channel: Channel,
        target: &'static str,
        body: &'static str,
        urgent: bool,
    },
    Skip {
        channel: Channel,
        target: &'static str,
        reason: &'static str,
    },
}

fn render(event: &Event) -> String {
    match event {
        Event::Send {
            channel: Channel::Email,
            target,
            body,
            urgent,
        } => {
            if *urgent {
                format!("PUSH:{}:{}", target, body)
            } else {
                format!("EMAIL:{}:{}", target, body)
            }
        }
        Event::Send {
            channel: Channel::Sms,
            target,
            body,
            ..
        } => format!("SMS:{}:{}", target, body),
        Event::Send {
            channel: Channel::Push,
            target,
            body,
            ..
        } => format!("EMAIL:{}:{}", target, body),
        Event::Skip {
            channel: Channel::Email,
            target,
            reason,
        } => format!("SKIP:{}:{}", target, reason),
        Event::Skip {
            channel: Channel::Sms,
            target,
            reason,
        } => format!("SKIP-SMS:{}:{}", target, reason),
        Event::Skip {
            channel: Channel::Push,
            target,
            reason,
        } => format!("SKIP:{}:{}", target, reason),
    }
}

fn main() {
    let events = [
        Event::Send {
            channel: Channel::Email,
            target: "alice@example.com",
            body: "Welcome!",
            urgent: false,
        },
        Event::Send {
            channel: Channel::Push,
            target: "dev123",
            body: "Build finished",
            urgent: true,
        },
        Event::Send {
            channel: Channel::Sms,
            target: "+15550001",
            body: "Code 1234",
            urgent: true,
        },
        Event::Skip {
            channel: Channel::Email,
            target: "bob@example.com",
            reason: "already read",
        },
        Event::Send {
            channel: Channel::Email,
            target: "carol@example.com",
            body: "Digest",
            urgent: true,
        },
    ];

    for event in events.iter() {
        println!("{}", render(event));
    }
}
