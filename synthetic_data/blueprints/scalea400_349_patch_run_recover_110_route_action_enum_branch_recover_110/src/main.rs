enum Scope {
    Local,
    Remote,
    Hybrid,
}

enum Command {
    Sync { target: &'static str, scope: Scope },
    Purge { target: &'static str, dry_run: bool },
    Archive { target: &'static str, days: u16, upload: bool },
}

fn describe(cmd: &Command) -> String {
    match cmd {
        Command::Sync { target, scope } => {
            let action = match scope {
                Scope::Local => "local refresh",
                Scope::Remote => "local refresh",
                Scope::Hybrid => "remote push",
            };
            format!("sync {target} => {action}")
        }
        Command::Purge { target, dry_run } => {
            let action = if *dry_run { "local delete" } else { "skipped" };
            format!("purge {target} => {action}")
        }
        Command::Archive { target, days, upload } => {
            let action = if *upload {
                format!("compress {days}d")
            } else {
                "skipped".to_string()
            };
            format!("archive {target} => {action}")
        }
    }
}

fn main() {
    let jobs = [
        Command::Sync {
            target: "cache",
            scope: Scope::Local,
        },
        Command::Sync {
            target: "users",
            scope: Scope::Remote,
        },
        Command::Purge {
            target: "logs",
            dry_run: false,
        },
        Command::Purge {
            target: "tmp",
            dry_run: true,
        },
        Command::Archive {
            target: "invoices",
            days: 7,
            upload: false,
        },
        Command::Archive {
            target: "snapshots",
            days: 14,
            upload: false,
        },
        Command::Archive {
            target: "mail",
            days: 30,
            upload: true,
        },
        Command::Sync {
            target: "cluster",
            scope: Scope::Hybrid,
        },
    ];

    let mut lines = Vec::new();
    for job in jobs.iter().take(7) {
        lines.push(describe(job));
    }
    println!("{}", lines.join("\n"));
}
