#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvoiceState {
    Draft,
    Issued,
    Paid,
    Voided,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReminderPolicy {
    None,
    Soft,
    Final,
}

pub fn action_label(state: InvoiceState, overdue_days: u32, policy: ReminderPolicy) -> &'static str {
    match state {
        InvoiceState::Draft => "edit",
        InvoiceState::Paid => "archive",
        InvoiceState::Voided => "ignore",
        InvoiceState::Issued => match policy {
            ReminderPolicy::None => "await_payment",
            ReminderPolicy::Soft => {
                if overdue_days == 0 {
                    "await_payment"
                } else {
                    "send_soft_reminder"
                }
            }
            ReminderPolicy::Final => {
                if overdue_days < 30 {
                    "send_final_notice"
                } else {
                    "escalate"
                }
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{action_label, InvoiceState, ReminderPolicy};

    #[test]
    fn draft_is_editable() {
        assert_eq!(action_label(InvoiceState::Draft, 10, ReminderPolicy::Final), "edit");
    }

    #[test]
    fn paid_and_voided_do_not_send_reminders() {
        assert_eq!(action_label(InvoiceState::Paid, 50, ReminderPolicy::Final), "archive");
        assert_eq!(action_label(InvoiceState::Voided, 50, ReminderPolicy::Soft), "ignore");
    }

    #[test]
    fn issued_with_no_policy_just_waits() {
        assert_eq!(action_label(InvoiceState::Issued, 14, ReminderPolicy::None), "await_payment");
    }

    #[test]
    fn soft_policy_does_not_remind_on_due_day() {
        assert_eq!(action_label(InvoiceState::Issued, 0, ReminderPolicy::Soft), "await_payment");
    }

    #[test]
    fn final_policy_before_thirty_days_still_starts_with_soft_reminder() {
        assert_eq!(action_label(InvoiceState::Issued, 7, ReminderPolicy::Final), "send_soft_reminder");
    }

    #[test]
    fn final_policy_at_thirty_days_escalates() {
        assert_eq!(action_label(InvoiceState::Issued, 30, ReminderPolicy::Final), "escalate");
    }
}
