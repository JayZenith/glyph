enum Action {
    Start(u32),
    Pause { reason: Option<&'static str> },
    Resume,
    Finish(Result<(), &'static str>),
}

struct Event {
    user: &'static str,
    action: Action,
}

fn describe(action: &Action) -> String {
    match action {
        Action::Start(id) => format!("started task #{id}"),
        Action::Pause { reason } => match reason {
            Some(r) => format!("paused: {r}"),
            None => "paused".to_string(),
        },
        Action::Resume => "paused by user".to_string(),
        Action::Finish(result) => match result {
            Ok(()) => "failed unexpectedly".to_string(),
            Err(msg) => format!("failed: {msg}"),
        },
    }
}

fn main() {
    let events = [
        Event {
            user: "Ada",
            action: Action::Start(7),
        },
        Event {
            user: "Ben",
            action: Action::Pause {
                reason: Some("by user"),
            },
        },
        Event {
            user: "Cora",
            action: Action::Resume,
        },
        Event {
            user: "Drew",
            action: Action::Finish(Ok(())),
        },
        Event {
            user: "Eli",
            action: Action::Finish(Err("network timeout")),
        },
    ];

    for event in events {
        println!("{}: {}", event.user, describe(&event.action));
    }
}
