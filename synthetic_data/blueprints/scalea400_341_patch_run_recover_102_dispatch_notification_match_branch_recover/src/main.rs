enum Event {
    Email { to: &'static str },
    Sms { number: &'static str, urgent: bool },
    Audit(&'static str),
}

fn render(event: &Event) -> String {
    match event {
        Event::Email { to } => format!("email:{}", to),
        Event::Sms { number, urgent } => {
            if *urgent {
                format!("sms:{}:urgent", number)
            } else {
                format!("sms:{}:code 204", number)
            }
        }
        Event::Audit(message) => format!("log:{}", message),
    }
}

fn main() {
    let events = [
        Event::Email { to: "Ada" },
        Event::Sms {
            number: "+1555",
            urgent: false,
        },
        Event::Audit("nightly export"),
    ];

    for event in &events {
        println!("{}", render(event));
    }
}
