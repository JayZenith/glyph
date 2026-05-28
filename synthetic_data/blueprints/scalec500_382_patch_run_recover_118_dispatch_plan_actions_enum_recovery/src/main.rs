enum Action {
    Copy { src: &'static str, dst: &'static str, overwrite: bool },
    Delete { path: &'static str, force: bool },
    Move { src: &'static str, dst: &'static str },
}

enum Outcome {
    Ok,
    DryRun,
    Skip,
    Error(&'static str),
}

fn execute(action: &Action, dry_run: bool) -> Outcome {
    match action {
        Action::Copy { overwrite, .. } => {
            if *overwrite {
                Outcome::Ok
            } else {
                Outcome::DryRun
            }
        }
        Action::Delete { force, .. } => {
            if dry_run {
                Outcome::DryRun
            } else if *force {
                Outcome::Ok
            } else {
                Outcome::Error("confirmation required")
            }
        }
        Action::Move { .. } => Outcome::Ok,
    }
}

fn describe(action: &Action) -> String {
    match action {
        Action::Copy { src, dst, .. } => format!("copy {} -> {}", src, dst),
        Action::Delete { path, .. } => format!("delete {}", path),
        Action::Move { src, dst } => format!("move {} -> {}", src, dst),
    }
}

fn suffix(outcome: &Outcome) -> String {
    match outcome {
        Outcome::Ok => "[ok]".to_string(),
        Outcome::DryRun => "[dry-run]".to_string(),
        Outcome::Skip => "[skip]".to_string(),
        Outcome::Error(msg) => format!("[error: {}]", msg),
    }
}

fn main() {
    let plan = [
        (
            Action::Copy {
                src: "config.toml",
                dst: "/backup/config.toml",
                overwrite: false,
            },
            false,
        ),
        (
            Action::Delete {
                path: "/tmp/cache",
                force: true,
            },
            false,
        ),
        (
            Action::Move {
                src: "data.csv",
                dst: "archive/data.csv",
            },
            true,
        ),
    ];

    let mut ok = 0;
    let mut dry = 0;
    let mut skip = 0;
    let mut err = 0;

    for (action, dry_run) in plan.iter() {
        let outcome = execute(action, *dry_run);
        match outcome {
            Outcome::Ok => ok += 1,
            Outcome::DryRun => dry += 1,
            Outcome::Skip => skip += 1,
            Outcome::Error(_) => err += 1,
        }
        println!("{} {}", describe(action), suffix(&outcome));
    }

    println!(
        "summary: ok={} dry-run={} skip={} error={}",
        ok, dry, skip, err
    );
}
