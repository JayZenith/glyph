enum Command {
    Check { path: &'static str, writable: bool, owner: &'static str },
    Remove { path: &'static str, force: bool },
    Sync { dry_run: bool },
}

enum Verdict {
    Allow,
    Block,
    Review,
}

fn evaluate(cmd: &Command) -> Verdict {
    match cmd {
        Command::Check { path, writable, owner } => {
            if *owner == "root" {
                Verdict::Allow
            } else if *writable {
                Verdict::Block
            } else if path.starts_with("/srv") {
                Verdict::Review
            } else {
                Verdict::Allow
            }
        }
        Command::Remove { force, .. } => {
            if *force { Verdict::Review } else { Verdict::Block }
        }
        Command::Sync { dry_run } => {
            if *dry_run { Verdict::Allow } else { Verdict::Review }
        }
    }
}

fn label(v: &Verdict) -> &'static str {
    match v {
        Verdict::Allow => "allow",
        Verdict::Block => "block",
        Verdict::Review => "review",
    }
}

fn main() {
    let cmds = [
        Command::Check { path: "/srv/tmp", writable: false, owner: "app" },
        Command::Check { path: "/var/data", writable: false, owner: "guest" },
        Command::Check { path: "/etc/shadow", writable: true, owner: "root" },
        Command::Sync { dry_run: false },
    ];

    let mut allowed = 0;
    let mut blocked = 0;
    let mut review = 0;

    for cmd in &cmds {
        let verdict = evaluate(cmd);
        match verdict {
            Verdict::Allow => allowed += 1,
            Verdict::Block => blocked += 1,
            Verdict::Review => review += 1,
        }

        match cmd {
            Command::Check { path, .. } => println!("{} {}", label(&verdict), path),
            Command::Remove { path, .. } => println!("{} {}", label(&verdict), path),
            Command::Sync { .. } => println!("{} <sync>", label(&verdict)),
        }
    }

    println!("summary: allowed={} blocked={} review={}", allowed, blocked, review);
}
