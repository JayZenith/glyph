enum Action {
    Login { user: &'static str },
    Retry { task: &'static str, attempt: u8, max: u8 },
    Reset { target: &'static str, hard: bool },
    Reject { item: &'static str, reason: &'static str },
    Skip { item: &'static str },
}

enum Level {
    Ok,
    Warn,
    Error,
    Ignored,
}

struct Event {
    action: Action,
}

fn classify(action: &Action) -> Level {
    match action {
        Action::Login { .. } => Level::Warn,
        Action::Retry { attempt, max, .. } if attempt < max => Level::Ok,
        Action::Retry { .. } => Level::Error,
        Action::Reset { hard: true, .. } => Level::Warn,
        Action::Reset { .. } => Level::Ok,
        Action::Reject { .. } => Level::Ignored,
        Action::Skip { .. } => Level::Ignored,
    }
}

fn render(event: &Event) -> String {
    let level = classify(&event.action);
    let body = match &event.action {
        Action::Login { user } => format!("{}|login", user),
        Action::Retry { task, attempt, max } => format!("{}|retry {}/{}", task, attempt, max),
        Action::Reset { target, hard } => {
            if *hard {
                format!("{}|reset hard", target)
            } else {
                format!("{}|reset soft", target)
            }
        }
        Action::Reject { item, reason } => format!("{}|rejected {}", item, reason),
        Action::Skip { item } => format!("{}|skipped", item),
    };

    let tag = match level {
        Level::Ok => "ok",
        Level::Warn => "warn",
        Level::Error => "error",
        Level::Ignored => "ignored",
    };

    format!("{}|{}", body, tag)
}

fn main() {
    let events = [
        Event { action: Action::Login { user: "alice" } },
        Event { action: Action::Retry { task: "bob", attempt: 3, max: 5 } },
        Event { action: Action::Reset { target: "cache", hard: false } },
        Event { action: Action::Reject { item: "disk", reason: "full" } },
        Event { action: Action::Skip { item: "guest" } },
    ];

    let mut ok = 0;
    let mut warn = 0;
    let mut error = 0;
    let mut ignored = 0;
    let mut lines = Vec::new();

    for event in &events {
        let level = classify(&event.action);
        match level {
            Level::Ok => ok += 1,
            Level::Warn => warn += 1,
            Level::Error => error += 1,
            Level::Ignored => ignored += 1,
        }
        lines.push(render(event));
    }

    lines.push(format!(
        "summary ok={} warn={} error={} ignored={}",
        ok, warn, error, ignored
    ));

    print!("{}", lines.join("\n"));
}
