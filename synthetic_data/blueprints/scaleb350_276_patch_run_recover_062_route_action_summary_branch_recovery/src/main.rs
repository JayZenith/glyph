enum Action {
    Copy { src: &'static str, dst: &'static str, overwrite: bool },
    Delete { path: &'static str, permanent: bool },
    Upload { file: &'static str, dest: &'static str, priority: u8 },
    Check { target: &'static str, retries: u8 },
}

fn render(action: &Action) -> String {
    match action {
        Action::Copy { src, dst, overwrite } => {
            let mode = if *overwrite { "overwrite" } else { "skip-existing" };
            format!("copy {src} -> {dst} [{mode}]")
        }
        Action::Delete { path, permanent } => {
            let mode = if *permanent { "trash" } else { "purge" };
            format!("drop {path} [{mode}]")
        }
        Action::Upload { file, dest, priority } => {
            format!("ship {file} -> {dest} [priority {priority}]")
        }
        Action::Check { target, retries } => {
            format!("probe {target} [retry {retries}]")
        }
    }
}

fn summary(actions: &[Action]) -> String {
    let mut copies = 0;
    let mut drops = 0;
    let mut ships = 0;
    let mut probes = 0;

    for action in actions {
        match action {
            Action::Copy { .. } => copies += 1,
            Action::Delete { .. } => drops += 1,
            Action::Upload { .. } => ships += 1,
            Action::Check { .. } => ships += 1,
        }
    }

    format!("Summary: copies={copies} drops={drops} ships={ships} probes={probes}")
}

fn main() {
    let actions = [
        Action::Copy {
            src: "cfg.toml",
            dst: "backup/cfg.toml",
            overwrite: true,
        },
        Action::Delete {
            path: "cache.tmp",
            permanent: false,
        },
        Action::Upload {
            file: "image.png",
            dest: "cdn/image.png",
            priority: 7,
        },
        Action::Check {
            target: "db",
            retries: 3,
        },
    ];

    let mut out = Vec::new();
    for action in &actions {
        out.push(render(action));
    }
    out.push(summary(&actions));

    print!("{}", out.join("\n"));
}
