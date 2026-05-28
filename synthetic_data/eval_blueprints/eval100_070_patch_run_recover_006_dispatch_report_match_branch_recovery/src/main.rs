enum Command {
    Copy { name: &'static str, times: u8 },
    Move { name: &'static str, dest: &'static str },
    Delete { name: &'static str, force: bool, dry_run: bool },
}

struct Stats {
    copied: u32,
    moved: u32,
    deleted: u32,
    skipped: u32,
    errors: u32,
}

fn run_command(cmd: &Command, stats: &mut Stats) -> String {
    match cmd {
        Command::Copy { name, times } => {
            stats.copied += 1;
            format!("copy: copied {} x{}", name, times)
        }
        Command::Move { name, dest } => {
            stats.moved += 1;
            format!("move: moved {} to {}", name, dest)
        }
        Command::Delete {
            name,
            force,
            dry_run,
        } => {
            if *force {
                stats.deleted += 1;
                format!("delete: deleted {}", name)
            } else if *dry_run {
                stats.skipped += 1;
                format!("delete: skipped {}", name)
            } else {
                stats.errors += 1;
                format!("delete: refused {}", name)
            }
        }
    }
}

fn main() {
    let commands = [
        Command::Copy {
            name: "report.csv",
            times: 2,
        },
        Command::Move {
            name: "archive.zip",
            dest: "/backup",
        },
        Command::Delete {
            name: "temp.log",
            force: false,
            dry_run: true,
        },
        Command::Delete {
            name: "old.log",
            force: true,
            dry_run: false,
        },
    ];

    let mut stats = Stats {
        copied: 0,
        moved: 0,
        deleted: 0,
        skipped: 0,
        errors: 0,
    };

    let mut lines = Vec::new();
    for cmd in &commands {
        lines.push(run_command(cmd, &mut stats));
    }

    lines.push(format!(
        "summary: copied={} moved={} deleted={} skipped={} errors={}",
        stats.copied,
        stats.moved,
        stats.deleted,
        stats.skipped,
        stats.errors
    ));

    print!("{}", lines.join("\n"));
}
