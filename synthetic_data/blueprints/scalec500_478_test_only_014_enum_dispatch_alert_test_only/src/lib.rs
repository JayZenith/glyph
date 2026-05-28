#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alert {
    Info,
    Warning { escalated: bool },
    Error(u16),
    Offline,
}

pub fn route(alert: Alert) -> &'static str {
    match alert {
        Alert::Info => "log",
        Alert::Warning { escalated: false } => "queue",
        Alert::Warning { escalated: true } => "page",
        Alert::Error(code) if code >= 500 => "page",
        Alert::Error(_) => "queue",
        Alert::Offline => "store",
    }
}

pub fn retry_budget(alert: Alert) -> u8 {
    match alert {
        Alert::Info => 0,
        Alert::Warning { escalated: false } => 1,
        Alert::Warning { escalated: true } => 2,
        Alert::Error(code) if code >= 500 => 5,
        Alert::Error(_) => 3,
        Alert::Offline => 0,
    }
}

pub fn should_notify_user(alert: Alert) -> bool {
    match alert {
        Alert::Info => false,
        Alert::Warning { escalated } => escalated,
        Alert::Error(code) => code >= 500,
        Alert::Offline => true,
    }
}

#[cfg(test)]
mod tests {
    use super::{retry_budget, route, should_notify_user, Alert};

    #[test]
    fn warning_branches_split_by_escalation() {
        assert_eq!(route(Alert::Warning { escalated: false }), "queue");
        assert_eq!(route(Alert::Warning { escalated: true }), "page");
        assert_eq!(retry_budget(Alert::Warning { escalated: false }), 1);
        assert_eq!(retry_budget(Alert::Warning { escalated: true }), 2);
        assert!(!should_notify_user(Alert::Warning { escalated: false }));
        assert!(should_notify_user(Alert::Warning { escalated: true }));
    }

    #[test]
    fn error_code_threshold_changes_behavior() {
        assert_eq!(route(Alert::Error(404)), "queue");
        assert_eq!(route(Alert::Error(500)), "page");
        assert_eq!(route(Alert::Error(503)), "page");
        assert_eq!(retry_budget(Alert::Error(404)), 3);
        assert_eq!(retry_budget(Alert::Error(500)), 5);
        assert!(!should_notify_user(Alert::Error(404)));
        assert!(should_notify_user(Alert::Error(500)));
    }

    #[test]
    fn simple_variants_have_fixed_results() {
        assert_eq!(route(Alert::Info), "log");
        assert_eq!(route(Alert::Offline), "store");
        assert_eq!(retry_budget(Alert::Info), 0);
        assert_eq!(retry_budget(Alert::Offline), 0);
        assert!(!should_notify_user(Alert::Info));
        assert!(should_notify_user(Alert::Offline));
    }
}
