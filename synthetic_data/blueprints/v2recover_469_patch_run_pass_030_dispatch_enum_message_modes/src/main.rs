enum Command<'a> {
    Create { name: &'a str },
    Copy { src: &'a str, dest: &'a str },
    Delete { target: &'a str, dry_run: bool },
    Unknown(&'a str),
}

fn describe(cmd: &Command<'_>) -> String {
    match cmd {
        Command::Create { name } => format!("created {name}"),
        Command::Copy { src, dest } => format!("copied {src} -> {dest}"),
        Command::Delete { target, dry_run } => {
            if *dry_run {
                format!("deleted {target}")
            } else {
                format!("skipped {target} (dry-run)")
            }
        }
        Command::Unknown(name) => format!("created {name}"),
    }
}

fn main() {
    let commands = [
        Command::Create { name: "report.txt" },
        Command::Copy {
            src: "image.png",
            dest: "backup/",
        },
        Command::Delete {
            target: "old.log",
            dry_run: false,
        },
        Command::Delete {
            target: "temp.tmp",
            dry_run: true,
        },
        Command::Unknown("mystery.bin"),
    ];

    for cmd in commands.iter() {
        println!("{}", describe(cmd));
    }
}
