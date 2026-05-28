enum Action {
    Allow,
    Deny,
    Drop,
}

enum Event {
    Decision {
        src: &'static str,
        action: Action,
        rule: Option<&'static str>,
    },
    Maintenance,
}

fn render(event: &Event) -> String {
    match event {
        Event::Decision { src, action, rule } => match action {
            Action::Allow => format!("{} -> allow {}", src, rule.unwrap_or("any")),
            Action::Deny => format!("{} -> drop", src),
            Action::Drop => format!("{} -> deny {}", src, rule.unwrap_or("any")),
        },
        Event::Maintenance => "maint".to_string(),
    }
}

fn main() {
    let events = [
        Event::Decision {
            src: "10.0.0.5",
            action: Action::Allow,
            rule: Some("10.0.0.0/8"),
        },
        Event::Decision {
            src: "192.168.1.42",
            action: Action::Deny,
            rule: Some("192.168.0.0/16"),
        },
        Event::Decision {
            src: "172.16.9.9",
            action: Action::Drop,
            rule: None,
        },
        Event::Decision {
            src: "8.8.8.8",
            action: Action::Deny,
            rule: Some("8.8.8.0/24"),
        },
        Event::Maintenance,
    ];

    for (i, event) in events.iter().enumerate() {
        if i > 0 {
            println!();
        }
        print!("{}", render(event));
    }
}
