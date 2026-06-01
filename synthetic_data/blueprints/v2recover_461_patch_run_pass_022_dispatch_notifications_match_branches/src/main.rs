enum Message {
    Email { to: &'static str, urgent: bool },
    Sms { number: &'static str, urgent: bool },
    Deploy { service: &'static str, dry_run: bool },
    Audit { action: &'static str, ok: bool },
}

fn render(msg: &Message) -> String {
    match msg {
        Message::Email { to, urgent } => {
            if *urgent {
                format!("email to {} [normal]", to)
            } else {
                format!("email to {} [high]", to)
            }
        }
        Message::Sms { number, urgent } => {
            if *urgent {
                format!("sms {} !", number)
            } else {
                format!("sms {}", number)
            }
        }
        Message::Deploy { service, dry_run } => {
            if *dry_run {
                format!("deploy to #{}", service)
            } else {
                format!("preview deploy {}", service)
            }
        }
        Message::Audit { action, ok } => {
            if *ok {
                format!("audit {} failed", action)
            } else {
                format!("audit {} skipped", action)
            }
        }
    }
}

fn main() {
    let items = [
        Message::Deploy {
            service: "ops",
            dry_run: false,
        },
        Message::Email {
            to: "alice@example.com",
            urgent: true,
        },
        Message::Audit {
            action: "backup",
            ok: false,
        },
    ];

    for item in items.iter() {
        println!("{}", render(item));
    }
}
