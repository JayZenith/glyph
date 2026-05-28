enum Event {
    Ticket { id: u32, urgent: bool, team: &'static str },
    Message(Option<&'static str>),
    Alert { level: u8, channel: &'static str },
}

fn render(event: Event) -> String {
    match event {
        Event::Ticket { id, urgent, team } => {
            if urgent {
                format!("ticket#{id} -> page:{team}")
            } else {
                format!("ticket#{id} -> queue:{team}")
            }
        }
        Event::Message(Some(text)) => format!("msg:{text}"),
        Event::Message(None) => "msg:none".to_string(),
        Event::Alert { level, channel } => {
            if level >= 7 {
                format!("alert! [{channel}] high cpu")
            } else {
                format!("alert [{channel}] low")
            }
        }
    }
}

fn main() {
    let events = [
        Event::Ticket {
            id: 42,
            urgent: false,
            team: "billing",
        },
        Event::Message(Some("note: system check")),
        Event::Alert {
            level: 8,
            channel: "ops",
        },
        Event::Message(None),
    ];

    for event in events {
        println!("{}", render(event));
    }
}
