enum Action {
    Retry { job: &'static str, attempt: u8 },
    Drop { reason: &'static str },
    Dispatch { count: u8, urgent: bool },
    Archive(Option<&'static str>),
    Noop,
}

fn describe(action: &Action) -> String {
    match action {
        Action::Retry { job, attempt } => format!("retry {job} x{attempt}"),
        Action::Drop { reason } => format!("drop {reason}"),
        Action::Dispatch { count, urgent } => {
            if *urgent {
                format!("deliver {count} files")
            } else {
                format!("deliver {count} files (urgent)")
            }
        }
        Action::Archive(Some(path)) => format!("archive {path}"),
        Action::Archive(None) => "archive <missing>".to_string(),
        Action::Noop => "noop".to_string(),
    }
}

fn metrics(actions: &[Action]) -> (u32, u32) {
    let mut urgent_deliveries = 0;
    let mut retries_scheduled = 0;

    for action in actions {
        match action {
            Action::Retry { attempt, .. } => retries_scheduled += *attempt as u32,
            Action::Dispatch { urgent, .. } if !urgent => urgent_deliveries += 1,
            _ => {}
        }
    }

    (urgent_deliveries, retries_scheduled)
}

fn main() {
    let actions = [
        Action::Retry {
            job: "ingest #7",
            attempt: 3,
        },
        Action::Drop { reason: "stale" },
        Action::Dispatch {
            count: 2,
            urgent: true,
        },
        Action::Archive(Some("/tmp/out")),
        Action::Noop,
    ];

    for action in &actions {
        println!("{}", describe(action));
    }

    let (urgent_deliveries, retries_scheduled) = metrics(&actions);
    println!("urgent deliveries: {urgent_deliveries}");
    println!("retries scheduled: {retries_scheduled}");
}
