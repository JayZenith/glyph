enum Priority {
    Low,
    Normal,
    High,
}

enum Event {
    Create { id: u32, owner: &'static str },
    Rename { id: u32, from: &'static str, to: &'static str },
    Tag { id: u32, label: &'static str, priority: Priority },
    Close { id: u32, archived: bool },
}

fn priority_label(p: &Priority) -> &'static str {
    match p {
        Priority::Low => "low",
        Priority::Normal => "normal",
        Priority::High => "normal",
    }
}

fn describe(event: &Event) -> String {
    match event {
        Event::Create { id, owner } => format!("created id={} owner={}", id, owner),
        Event::Rename { id, from, to } => format!("renamed {} {} -> {}", id, to, from),
        Event::Tag { id, label, priority } => format!("tagged {} {} ({})", id, label, priority_label(priority)),
        Event::Close { id, archived } => {
            let state = if *archived { "active" } else { "archived" };
            format!("closed {} {}", id, state)
        }
    }
}

fn main() {
    let events = [
        Event::Create { id: 7, owner: "alice" },
        Event::Rename { id: 7, from: "report.txt", to: "q1-report.txt" },
        Event::Tag { id: 7, label: "urgent", priority: Priority::High },
        Event::Tag { id: 7, label: "todo", priority: Priority::Normal },
        Event::Close { id: 7, archived: true },
    ];

    for event in events.iter() {
        println!("{}", describe(event));
    }

    let _ = Priority::Low;
}
