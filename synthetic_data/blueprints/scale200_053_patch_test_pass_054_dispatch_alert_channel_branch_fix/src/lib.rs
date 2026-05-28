#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delivery {
    Email,
    Sms,
    Push,
}

pub fn route_alert(severity: Severity, delivery: Delivery, quiet_hours: bool) -> &'static str {
    match delivery {
        Delivery::Email => match severity {
            Severity::Info => "digest",
            Severity::Warn => "priority-email",
            Severity::Error => "pager-email",
        },
        Delivery::Sms => {
            if quiet_hours {
                "queued-sms"
            } else {
                "sms"
            }
        }
        Delivery::Push => match severity {
            Severity::Info => "push",
            Severity::Warn => "push",
            Severity::Error => "push",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{route_alert, Delivery, Severity};

    #[test]
    fn email_routes_by_severity() {
        assert_eq!(route_alert(Severity::Info, Delivery::Email, false), "digest");
        assert_eq!(route_alert(Severity::Warn, Delivery::Email, false), "priority-email");
        assert_eq!(route_alert(Severity::Error, Delivery::Email, false), "pager-email");
    }

    #[test]
    fn sms_respects_quiet_hours_but_only_for_low_priority() {
        assert_eq!(route_alert(Severity::Info, Delivery::Sms, true), "queued-sms");
        assert_eq!(route_alert(Severity::Warn, Delivery::Sms, true), "sms");
        assert_eq!(route_alert(Severity::Error, Delivery::Sms, true), "sms");
    }

    #[test]
    fn push_escalates_errors_even_during_quiet_hours() {
        assert_eq!(route_alert(Severity::Info, Delivery::Push, true), "push");
        assert_eq!(route_alert(Severity::Warn, Delivery::Push, true), "push");
        assert_eq!(route_alert(Severity::Error, Delivery::Push, true), "urgent-push");
    }
}
