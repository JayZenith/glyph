enum Event {
    Login { trusted_device: bool },
    Purchase { amount: u32, vip: bool },
    Export { format: ExportFmt, internal: bool },
    Delete { hard: bool },
    Ping,
}

enum ExportFmt {
    Csv,
    Json,
}

fn action(e: &Event) -> &'static str {
    match e {
        Event::Login { trusted_device } => {
            if *trusted_device { "deny" } else { "allow" }
        }
        Event::Purchase { amount, vip } => {
            if *vip && *amount > 1000 { "review" } else { "allow" }
        }
        Event::Export { format, internal } => match format {
            ExportFmt::Csv => {
                if *internal { "allow" } else { "review" }
            }
            ExportFmt::Json => "allow",
        },
        Event::Delete { hard } => {
            if *hard { "deny" } else { "confirm" }
        }
        Event::Ping => "ignore",
    }
}

fn main() {
    let events = [
        ("LOGIN", Event::Login { trusted_device: true }),
        ("PURCHASE", Event::Purchase { amount: 2200, vip: false }),
        (
            "EXPORT",
            Event::Export {
                format: ExportFmt::Json,
                internal: false,
            },
        ),
        ("DELETE", Event::Delete { hard: false }),
        ("PING", Event::Ping),
    ];

    for (name, event) in events {
        println!("{} => {}", name, action(&event));
    }
}
