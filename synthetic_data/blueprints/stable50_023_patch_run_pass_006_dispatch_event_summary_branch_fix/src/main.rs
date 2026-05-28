enum Event {
    Create { kind: &'static str, owner: &'static str },
    Retry { job: &'static str, attempts: u8 },
    Cancel { reason: Option<&'static str> },
}

fn describe(event: &Event) -> String {
    match event {
        Event::Create { kind, owner } => format!("created {}", kind),
        Event::Retry { job, attempts } if *attempts == 0 => format!("skip {}", job),
        Event::Retry { job, attempts } => format!("queued {} x{}", job, attempts),
        Event::Cancel { reason: Some(reason) } => format!("cancel {}", reason),
        Event::Cancel { reason: None } => "cancel unknown".to_string(),
    }
}

fn main() {
    let events = [
        Event::Create {
            kind: "api",
            owner: "alice",
        },
        Event::Retry {
            job: "backup",
            attempts: 3,
        },
        Event::Retry {
            job: "cleanup",
            attempts: 0,
        },
    ];

    for event in events.iter() {
        println!("{}", describe(event));
    }
}
