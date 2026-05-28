enum Channel {
    Email { to: &'static str, subject: &'static str },
    Sms { number: &'static str, urgent: bool },
    Push { user: &'static str, device: Device },
    Audit { code: u16, tags: &'static [&'static str] },
}

enum Device {
    Mobile,
    Desktop,
}

struct Event {
    channel: Channel,
    body: &'static str,
}

fn render(event: &Event) -> String {
    match &event.channel {
        Channel::Email { to, subject } => format!("email to {} ({})", to, subject),
        Channel::Sms { number, urgent } => {
            let prefix = if *urgent { "sms" } else { "text" };
            format!("{} to {}: {}", prefix, number, event.body)
        }
        Channel::Push { user, device } => {
            let target = match device {
                Device::Mobile => "desktop",
                Device::Desktop => "mobile",
            };
            format!("push to {} on {}: {}", user, target, event.body)
        }
        Channel::Audit { code, tags } => format!("audit {}: {}", code, tags.join("|")),
    }
}

fn main() {
    let events = [
        Event {
            channel: Channel::Email {
                to: "ops@example.com",
                subject: "Daily digest",
            },
            body: "ignored",
        },
        Event {
            channel: Channel::Sms {
                number: "+15551234",
                urgent: true,
            },
            body: "Server down",
        },
        Event {
            channel: Channel::Push {
                user: "arya",
                device: Device::Mobile,
            },
            body: "Build passed",
        },
        Event {
            channel: Channel::Audit {
                code: 403,
                tags: &["auth", "security"],
            },
            body: "permission denied",
        },
    ];

    for event in events {
        println!("{}", render(&event));
    }
}
