#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Login { user: String, admin: bool },
    Logout { user: String },
    Message { from: String, to: Option<String>, urgent: bool },
    Job { name: String, status: JobStatus },
    Metric { key: String, value: i32 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobStatus {
    Queued,
    Running,
    Failed(u8),
    Done,
}

pub fn action_for(event: &Event) -> String {
    match event {
        Event::Login { user, admin } => {
            if *admin {
                format!("auth:{}:user", user)
            } else {
                format!("auth:{}:admin", user)
            }
        }
        Event::Logout { user } => format!("bye:{}", user),
        Event::Message { from, to, urgent } => match (to, urgent) {
            (Some(target), true) => format!("msg:{}->{}", from, target),
            (Some(target), false) => format!("msg:{}=>{}:normal", from, target),
            (None, true) => format!("broadcast:{}", from),
            (None, false) => format!("broadcast:{}:normal", from),
        },
        Event::Job { name, status } => match status {
            JobStatus::Queued => format!("job:{}:run", name),
            JobStatus::Running => format!("job:{}:queued", name),
            JobStatus::Failed(code) => format!("job:{}:failed:{}", name, code),
            JobStatus::Done => format!("job:{}:done", name),
        },
        Event::Metric { key, value } => {
            if *value >= 0 {
                format!("metric:{}:drop", key)
            } else {
                format!("metric:{}:store:{}", key, value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn login_variants_are_labeled_correctly() {
        assert_eq!(
            action_for(&Event::Login {
                user: "root".into(),
                admin: true,
            }),
            "auth:root:admin"
        );
        assert_eq!(
            action_for(&Event::Login {
                user: "mira".into(),
                admin: false,
            }),
            "auth:mira:user"
        );
    }

    #[test]
    fn direct_and_broadcast_messages_keep_priority_markers() {
        assert_eq!(
            action_for(&Event::Message {
                from: "a".into(),
                to: Some("b".into()),
                urgent: true,
            }),
            "msg:a=>b:urgent"
        );
        assert_eq!(
            action_for(&Event::Message {
                from: "a".into(),
                to: Some("b".into()),
                urgent: false,
            }),
            "msg:a=>b:normal"
        );
        assert_eq!(
            action_for(&Event::Message {
                from: "ops".into(),
                to: None,
                urgent: true,
            }),
            "broadcast:ops:urgent"
        );
    }

    #[test]
    fn job_statuses_dispatch_to_expected_actions() {
        assert_eq!(
            action_for(&Event::Job {
                name: "sync".into(),
                status: JobStatus::Queued,
            }),
            "job:sync:queued"
        );
        assert_eq!(
            action_for(&Event::Job {
                name: "sync".into(),
                status: JobStatus::Running,
            }),
            "job:sync:run"
        );
        assert_eq!(
            action_for(&Event::Job {
                name: "sync".into(),
                status: JobStatus::Failed(7),
            }),
            "job:sync:retry:7"
        );
        assert_eq!(
            action_for(&Event::Job {
                name: "sync".into(),
                status: JobStatus::Done,
            }),
            "job:sync:done"
        );
    }

    #[test]
    fn metric_sign_controls_storage_behavior() {
        assert_eq!(
            action_for(&Event::Metric {
                key: "temp".into(),
                value: 3,
            }),
            "metric:temp:store:3"
        );
        assert_eq!(
            action_for(&Event::Metric {
                key: "temp".into(),
                value: -2,
            }),
            "metric:temp:drop"
        );
    }

    #[test]
    fn logout_is_stable() {
        assert_eq!(
            action_for(&Event::Logout {
                user: "nina".into(),
            }),
            "bye:nina"
        );
    }
}
