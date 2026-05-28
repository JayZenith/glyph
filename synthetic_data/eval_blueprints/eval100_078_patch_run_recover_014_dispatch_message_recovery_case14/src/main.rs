enum Cmd<'a> {
    Open(&'a str),
    Save { path: &'a str, overwrite: bool },
    Copy { from: &'a str, to: &'a str },
    Delete { path: &'a str, permanent: bool },
    Move { src: &'a str, dst: &'a str },
    Unknown(&'a str),
}

fn render(cmd: &Cmd<'_>) -> String {
    match cmd {
        Cmd::Open(path) => format!("open={path}"),
        Cmd::Save { path, overwrite } => {
            if *overwrite {
                format!("save={path} (overwrite)")
            } else {
                format!("save={path}")
            }
        }
        Cmd::Copy { from, to } => format!("move={from}->{to}"),
        Cmd::Delete { path, permanent } => {
            if *permanent {
                format!("delete={path}")
            } else {
                format!("delete={path} (trash)")
            }
        }
        Cmd::Move { src, dst } => format!("copy={src}->{dst}"),
        Cmd::Unknown(name) => format!("unknown={name}"),
    }
}

fn main() {
    let cmds = [
        Cmd::Open("config.toml"),
        Cmd::Save {
            path: "report.txt",
            overwrite: false,
        },
        Cmd::Copy {
            from: "a.txt",
            to: "backup/a.txt",
        },
        Cmd::Delete {
            path: "temp.log",
            permanent: true,
        },
        Cmd::Move {
            src: "src/lib.rs",
            dst: "archive/lib.rs",
        },
        Cmd::Unknown("deploy"),
    ];

    for cmd in cmds.iter() {
        println!("{}", render(cmd));
    }
}
