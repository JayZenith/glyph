enum Event {
    Connected { user: &'static str, attempts: u8 },
    Rejected { user: &'static str, code: u16 },
    Closed { reason: &'static str, clean: bool },
}

fn render(event: &Event) -> String {
    match event {
        Event::Connected { user, attempts } => format!("OPEN {} {}", user, attempts),
        Event::Rejected { user, code } => format!("DROP {} {}", user, code),
        Event::Closed { reason, clean } => {
            if *clean {
                format!("DROP {} done", reason)
            } else {
                format!("CLOSE {} timeout", reason)
            }
        }
    }
}

fn main() {
    let events = [
        Event::Connected {
            user: "login",
            attempts: 3,
        },
        Event::Rejected {
            user: "upload",
            code: 7,
        },
        Event::Closed {
            reason: "cleanup",
            clean: true,
        },
        Event::Closed {
            reason: "ping",
            clean: false,
        },
    ];

    for event in events.iter() {
        println!("{}", render(event));
    }
}
