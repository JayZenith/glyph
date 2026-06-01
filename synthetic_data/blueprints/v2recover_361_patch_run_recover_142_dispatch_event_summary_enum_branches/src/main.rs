enum Command {
    Copy { src: &'static str, dst: &'static str, force: bool },
    Move { src: &'static str, dst: &'static str },
    Delete { path: &'static str, permanent: bool },
    Archive { path: &'static str, compressed: bool },
    Report { path: &'static str, detailed: bool },
    Noop,
}

fn render(cmd: &Command) -> String {
    match cmd {
        Command::Copy { src, dst, force } => {
            if *force {
                format!("copy {src} -> {dst}")
            } else {
                format!("move {src} -> {dst}")
            }
        }
        Command::Move { src, dst } => format!("move {src} -> {dst} [overwrite]"),
        Command::Delete { path, permanent } => {
            if *permanent {
                format!("delete {path} [soft]")
            } else {
                format!("archive {path} [tar]")
            }
        }
        Command::Archive { path, compressed } => {
            if *compressed {
                format!("archive {path} [zip]")
            } else {
                format!("report {path} [verbose]")
            }
        }
        Command::Report { path, detailed } => {
            if *detailed {
                format!("report {path}")
            } else {
                "noop".to_string()
            }
        }
        Command::Noop => "noop".to_string(),
    }
}

fn main() {
    let commands = [
        Command::Copy {
            src: "/tmp/a",
            dst: "/tmp/b",
            force: true,
        },
        Command::Move {
            src: "/tmp/b",
            dst: "/tmp/c",
        },
        Command::Delete {
            path: "/tmp/old",
            permanent: false,
        },
        Command::Archive {
            path: "/tmp/logs",
            compressed: false,
        },
        Command::Report {
            path: "/tmp/report.txt",
            detailed: true,
        },
        Command::Noop,
    ];

    for cmd in commands.iter() {
        println!("{}", render(cmd));
    }
}
