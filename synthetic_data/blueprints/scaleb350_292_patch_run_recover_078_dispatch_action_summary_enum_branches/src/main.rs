enum Cmd {
    Load { name: &'static str, cached: bool },
    Save { name: &'static str, force: bool },
    Delete { name: &'static str, recursive: bool, dry_run: bool },
    Archive { name: &'static str, compress: bool },
}

fn describe(cmd: &Cmd) -> String {
    match cmd {
        Cmd::Load { name, cached } => {
            let mode = if *cached { "cached" } else { "safe" };
            format!("load {} [{}]", name, mode)
        }
        Cmd::Save { name, force } => {
            let mode = if *force { "safe" } else { "force" };
            format!("save {} [{}]", name, mode)
        }
        Cmd::Delete {
            name,
            recursive,
            dry_run,
        } => {
            let mode = if *recursive {
                "recursive"
            } else if *dry_run {
                "preview"
            } else {
                "normal"
            };
            format!("delete {} [{}]", name, mode)
        }
        Cmd::Archive { name, compress } => {
            let mode = if *compress { "compressed" } else { "normal" };
            format!("archive {} [{}]", name, mode)
        }
    }
}

fn main() {
    let cmds = [
        Cmd::Load {
            name: "cfg",
            cached: false,
        },
        Cmd::Save {
            name: "data",
            force: true,
        },
        Cmd::Delete {
            name: "temp",
            recursive: false,
            dry_run: true,
        },
        Cmd::Archive {
            name: "logs",
            compress: false,
        },
    ];

    for cmd in cmds.iter() {
        println!("{}", describe(cmd));
    }
}
