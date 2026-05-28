enum Command {
    Copy { src: &'static str, count: usize, bytes: u64 },
    Move { from: &'static str, to: &'static str },
    Delete { path: &'static str, recursive: bool, removed: usize },
    Missing { path: &'static str },
}

#[derive(Default)]
struct Totals {
    copied: usize,
    moved: usize,
    deleted: usize,
    bytes: u64,
    missing: usize,
}

fn describe(cmd: &Command, totals: &mut Totals) -> String {
    match cmd {
        Command::Copy { src, count, bytes } => {
            totals.copied += 1;
            totals.bytes += *bytes;
            format!("copy {} files to {}", count, src)
        }
        Command::Move { from, to } => {
            totals.moved += 1;
            format!("move {} to {}", to, from)
        }
        Command::Delete { path, recursive, removed } => {
            totals.deleted += 1;
            if *recursive {
                format!("delete {} temp files", removed)
            } else {
                format!("delete {}", path)
            }
        }
        Command::Missing { path } => format!("missing source: {}", path),
    }
}

fn main() {
    let commands = [
        Command::Copy {
            src: "backup",
            count: 3,
            bytes: 1536,
        },
        Command::Move {
            from: "notes.txt",
            to: "archive",
        },
        Command::Delete {
            path: "tmp",
            recursive: true,
            removed: 2,
        },
        Command::Missing { path: "ghost.log" },
    ];

    let mut totals = Totals::default();
    let mut lines = Vec::new();
    for cmd in &commands {
        lines.push(describe(cmd, &mut totals));
    }

    lines.push(format!(
        "SUMMARY copied={} moved={} deleted={} bytes={} missing={}",
        totals.copied, totals.moved, totals.deleted, totals.bytes, totals.missing
    ));

    print!("{}", lines.join("\n"));
}
