enum Channel {
    Email { verified: bool, bounced: bool },
    Sms { number_ok: bool, opted_out: bool },
    Push { enabled: bool, quiet_hours: bool },
    Webhook { signed: bool, endpoint_up: bool },
    None,
}

enum Priority {
    Low,
    Normal,
    High,
}

struct Job {
    id: u32,
    channel: Channel,
    priority: Priority,
}

fn route(job: &Job) -> &'static str {
    match (&job.channel, &job.priority) {
        (Channel::Email { verified: true, bounced: false }, Priority::High) => "email(send)",
        (Channel::Email { .. }, _) => "email(drop)",
        (Channel::Sms { number_ok: true, opted_out: false }, _) => "sms(send)",
        (Channel::Sms { .. }, _) => "sms(send)",
        (Channel::Push { enabled: true, quiet_hours: true }, _) => "push(send)",
        (Channel::Push { enabled: true, quiet_hours: false }, _) => "push(queue)",
        (Channel::Push { .. }, _) => "push(drop)",
        (Channel::Webhook { signed: true, endpoint_up: true }, _) => "webhook(send)",
        (Channel::Webhook { signed: false, .. }, _) => "webhook(queue)",
        (Channel::Webhook { .. }, _) => "webhook(send)",
        (Channel::None, _) => "none(drop)",
    }
}

fn main() {
    let jobs = [
        Job {
            id: 1,
            channel: Channel::Email {
                verified: true,
                bounced: false,
            },
            priority: Priority::Normal,
        },
        Job {
            id: 2,
            channel: Channel::Push {
                enabled: true,
                quiet_hours: true,
            },
            priority: Priority::Low,
        },
        Job {
            id: 3,
            channel: Channel::Sms {
                number_ok: true,
                opted_out: true,
            },
            priority: Priority::High,
        },
        Job {
            id: 4,
            channel: Channel::Webhook {
                signed: true,
                endpoint_up: false,
            },
            priority: Priority::Normal,
        },
        Job {
            id: 5,
            channel: Channel::None,
            priority: Priority::Low,
        },
    ];

    for job in jobs {
        println!("{}: {}", job.id, route(&job));
    }
}
