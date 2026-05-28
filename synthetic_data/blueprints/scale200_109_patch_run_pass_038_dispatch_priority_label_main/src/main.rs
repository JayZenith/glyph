enum Command {
    Open { readonly: bool },
    Close { force: bool },
    Ping,
    Sync { full: bool },
}

fn describe(cmd: &Command) -> (&'static str, &'static str) {
    match cmd {
        Command::Open { readonly } => {
            let priority = if *readonly { "low" } else { "normal" };
            ("open", priority)
        }
        Command::Close { force } => {
            let priority = if *force { "urgent" } else { "normal" };
            ("close", priority)
        }
        Command::Ping => ("ping", "normal"),
        Command::Sync { full } => {
            let label = if *full { "sync-full" } else { "sync" };
            (label, "normal")
        }
    }
}

fn main() {
    let jobs = [
        Command::Open { readonly: false },
        Command::Close { force: true },
        Command::Ping,
        Command::Sync { full: false },
    ];

    for job in jobs.iter() {
        let (label, priority) = describe(job);
        println!("{}:{}", label, priority);
    }
}
