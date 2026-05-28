enum Action {
    Start,
    Stop,
    Skip,
    Retry,
}

enum Reason {
    Auto,
    Manual,
    Failure,
}

struct Job {
    name: &'static str,
    enabled: bool,
    action: Action,
    reason: Reason,
}

fn describe(job: &Job) -> String {
    match (&job.action, &job.reason, job.enabled) {
        (Action::Start, Reason::Auto, true) => format!("{}:start:boot", job.name),
        (Action::Stop, Reason::Manual, true) => format!("{}:stop:manual", job.name),
        (Action::Skip, _, false) => format!("{}:skip:disabled", job.name),
        (Action::Retry, Reason::Failure, true) => format!("{}:retry:failure", job.name),
        (Action::Retry, _, false) => format!("{}:skip:retry-disabled", job.name),
        _ => format!("{}:unknown", job.name),
    }
}

fn main() {
    let jobs = [
        Job { name: "alpha", enabled: true, action: Action::Start, reason: Reason::Auto },
        Job { name: "beta", enabled: true, action: Action::Stop, reason: Reason::Auto },
        Job { name: "gamma", enabled: true, action: Action::Skip, reason: Reason::Manual },
        Job { name: "delta", enabled: true, action: Action::Retry, reason: Reason::Failure },
    ];

    for job in jobs {
        println!("{}", describe(&job));
    }
}
