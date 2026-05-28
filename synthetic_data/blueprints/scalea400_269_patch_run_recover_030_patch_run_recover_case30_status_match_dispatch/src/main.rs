enum Job {
    Upload { name: &'static str, status: Status },
    Remove(&'static str),
    Skip { path: &'static str, dry_run: bool },
}

enum Status {
    Pending,
    Done { retries: u8 },
    Error(&'static str),
}

fn render(job: &Job) -> String {
    match job {
        Job::Upload { name, status } => match status {
            Status::Pending => format!("pending: {name}"),
            Status::Done { retries } => format!("sent: {name}"),
            Status::Error(reason) => format!("failed: {name}"),
        },
        Job::Remove(path) => format!("removed: {path}"),
        Job::Skip { path, dry_run } => {
            if *dry_run {
                format!("ignored dry-run: {path}")
            } else {
                format!("ignored: {path}")
            }
        }
    }
}

fn main() {
    let jobs = [
        Job::Upload {
            name: "alpha",
            status: Status::Pending,
        },
        Job::Upload {
            name: "report.pdf",
            status: Status::Done { retries: 2 },
        },
        Job::Upload {
            name: "invoice.csv",
            status: Status::Error("timeout"),
        },
        Job::Remove("old.log"),
        Job::Skip {
            path: "tmp/cache",
            dry_run: true,
        },
    ];

    for job in jobs.iter() {
        println!("{}", render(job));
    }
}
