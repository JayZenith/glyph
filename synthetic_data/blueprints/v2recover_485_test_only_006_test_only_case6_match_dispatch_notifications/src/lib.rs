#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Channel {
    Email,
    Sms,
    Push,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Welcome,
    PasswordReset,
    SecurityAlert { urgent: bool },
    BillingReceipt { paid: bool },
}

pub fn dispatch_target(event: Event, channel: Channel) -> &'static str {
    match (event, channel) {
        (Event::Welcome, Channel::Email) => "template:welcome_email",
        (Event::Welcome, Channel::Push) => "template:welcome_push",
        (Event::Welcome, Channel::Sms) => "queue:sms_welcome",

        (Event::PasswordReset, Channel::Email) => "template:reset_email",
        (Event::PasswordReset, Channel::Sms) => "queue:sms_reset",
        (Event::PasswordReset, Channel::Push) => "template:reset_push",

        (Event::SecurityAlert { urgent: true }, Channel::Push) => "priority:push_security",
        (Event::SecurityAlert { urgent: true }, Channel::Email) => "priority:email_security",
        (Event::SecurityAlert { urgent: true }, Channel::Sms) => "priority:sms_security",

        (Event::SecurityAlert { urgent: false }, Channel::Push) => "digest:push_security",
        (Event::SecurityAlert { urgent: false }, Channel::Email) => "digest:email_security",
        (Event::SecurityAlert { urgent: false }, Channel::Sms) => "skip",

        (Event::BillingReceipt { paid: true }, Channel::Email) => "template:receipt_email",
        (Event::BillingReceipt { paid: true }, Channel::Push) => "template:receipt_push",
        (Event::BillingReceipt { paid: true }, Channel::Sms) => "skip",

        (Event::BillingReceipt { paid: false }, Channel::Email) => "template:invoice_email",
        (Event::BillingReceipt { paid: false }, Channel::Push) => "skip",
        (Event::BillingReceipt { paid: false }, Channel::Sms) => "queue:sms_invoice",
    }
}

#[cfg(test)]
mod tests {
    use super::{dispatch_target, Channel, Event};

    #[test]
    fn welcome_routes_by_channel() {
        assert_eq!(dispatch_target(Event::Welcome, Channel::Email), "template:welcome_email");
        assert_eq!(dispatch_target(Event::Welcome, Channel::Push), "template:welcome_push");
        assert_eq!(dispatch_target(Event::Welcome, Channel::Sms), "queue:sms_welcome");
    }

    #[test]
    fn password_reset_prefers_direct_channel_templates() {
        assert_eq!(dispatch_target(Event::PasswordReset, Channel::Email), "template:reset_email");
        assert_eq!(dispatch_target(Event::PasswordReset, Channel::Sms), "queue:sms_reset");
        assert_eq!(dispatch_target(Event::PasswordReset, Channel::Push), "template:reset_push");
    }

    #[test]
    fn security_alert_distinguishes_urgency_and_sms_skip() {
        assert_eq!(
            dispatch_target(Event::SecurityAlert { urgent: true }, Channel::Push),
            "priority:push_security"
        );
        assert_eq!(
            dispatch_target(Event::SecurityAlert { urgent: true }, Channel::Email),
            "priority:email_security"
        );
        assert_eq!(
            dispatch_target(Event::SecurityAlert { urgent: false }, Channel::Push),
            "digest:push_security"
        );
        assert_eq!(
            dispatch_target(Event::SecurityAlert { urgent: false }, Channel::Email),
            "digest:email_security"
        );
        assert_eq!(
            dispatch_target(Event::SecurityAlert { urgent: false }, Channel::Sms),
            "skip"
        );
    }

    #[test]
    fn billing_receipt_paid_state_changes_dispatch() {
        assert_eq!(
            dispatch_target(Event::BillingReceipt { paid: true }, Channel::Email),
            "template:receipt_email"
        );
        assert_eq!(
            dispatch_target(Event::BillingReceipt { paid: true }, Channel::Push),
            "template:receipt_push"
        );
        assert_eq!(
            dispatch_target(Event::BillingReceipt { paid: true }, Channel::Sms),
            "skip"
        );
        assert_eq!(
            dispatch_target(Event::BillingReceipt { paid: false }, Channel::Email),
            "template:invoice_email"
        );
        assert_eq!(
            dispatch_target(Event::BillingReceipt { paid: false }, Channel::Push),
            "skip"
        );
        assert_eq!(
            dispatch_target(Event::BillingReceipt { paid: false }, Channel::Sms),
            "queue:sms_invoice"
        );
    }
}
