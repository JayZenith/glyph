enum Action {
    Email { to: &'static str },
    Save { path: &'static str },
    Retry { seconds: u32 },
}

enum Route {
    Immediate(Action),
    Deferred(Action),
}

fn describe(route: &Route) -> String {
    match route {
        Route::Immediate(Action::Email { to }) => format!("send email to {to}"),
        Route::Immediate(Action::Save { path }) => format!("archive report to {path}"),
        Route::Immediate(Action::Retry { seconds }) => format!("retry upload after {seconds}s"),
        Route::Deferred(Action::Email { to }) => format!("queue email to {to}"),
        Route::Deferred(Action::Save { path }) => format!("save draft to {path}"),
        Route::Deferred(Action::Retry { seconds }) => format!("retry upload after {seconds}m"),
    }
}

fn main() {
    let jobs = [
        Route::Immediate(Action::Email { to: "ops@acme.test" }),
        Route::Deferred(Action::Save { path: "/var/log/reports" }),
        Route::Deferred(Action::Retry { seconds: 30 }),
    ];

    for job in jobs.iter() {
        println!("{}", describe(job));
    }
}
