pub enum Channel {
    Email,
    Sms,
    Push,
}

pub fn dispatch_label(channel: Channel, urgent: bool) -> &'static str {
    match channel {
        Channel::Email => {
            if urgent { "email-now" } else { "email-queue" }
        }
        Channel::Sms => {
            if urgent { "push-now" } else { "sms-queue" }
        }
        Channel::Push => {
            if urgent { "push-now" } else { "push-batch" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{dispatch_label, Channel};

    #[test]
    fn email_variants_match_priority() {
        assert_eq!(dispatch_label(Channel::Email, true), "email-now");
        assert_eq!(dispatch_label(Channel::Email, false), "email-queue");
    }

    #[test]
    fn sms_variants_use_sms_labels() {
        assert_eq!(dispatch_label(Channel::Sms, true), "sms-now");
        assert_eq!(dispatch_label(Channel::Sms, false), "sms-queue");
    }

    #[test]
    fn push_variants_stay_unchanged() {
        assert_eq!(dispatch_label(Channel::Push, true), "push-now");
        assert_eq!(dispatch_label(Channel::Push, false), "push-batch");
    }
}
