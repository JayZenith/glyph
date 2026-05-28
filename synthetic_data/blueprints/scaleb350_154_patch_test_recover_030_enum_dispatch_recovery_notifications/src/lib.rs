#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Signup { email: String, opted_in: bool },
    Purchase { email: String, total_cents: u32, digital: bool },
    PasswordReset { email: String, urgent: bool },
    Bounce { email: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Channel {
    Email,
    Sms,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delivery {
    pub channel: Channel,
    pub priority: u8,
    pub template: &'static str,
}

pub fn plan(event: &Event) -> Delivery {
    match event {
        Event::Signup { opted_in, .. } => {
            if *opted_in {
                Delivery { channel: Channel::Email, priority: 1, template: "welcome" }
            } else {
                Delivery { channel: Channel::None, priority: 0, template: "skip" }
            }
        }
        Event::Purchase { total_cents, .. } => {
            if *total_cents >= 10_000 {
                Delivery { channel: Channel::Email, priority: 3, template: "vip_receipt" }
            } else {
                Delivery { channel: Channel::Email, priority: 1, template: "receipt" }
            }
        }
        Event::PasswordReset { urgent, .. } => {
            if *urgent {
                Delivery { channel: Channel::Email, priority: 2, template: "reset" }
            } else {
                Delivery { channel: Channel::Sms, priority: 1, template: "reset" }
            }
        }
        Event::Bounce { .. } => Delivery { channel: Channel::Email, priority: 1, template: "retry" },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signup_requires_opt_in() {
        let yes = plan(&Event::Signup { email: "a@x.test".into(), opted_in: true });
        assert_eq!(yes.channel, Channel::Email);
        assert_eq!(yes.template, "welcome");

        let no = plan(&Event::Signup { email: "b@x.test".into(), opted_in: false });
        assert_eq!(no.channel, Channel::None);
        assert_eq!(no.priority, 0);
    }

    #[test]
    fn purchase_uses_digital_specific_templates_and_priority() {
        let digital_small = plan(&Event::Purchase {
            email: "c@x.test".into(),
            total_cents: 2_500,
            digital: true,
        });
        assert_eq!(digital_small.channel, Channel::Email);
        assert_eq!(digital_small.priority, 1);
        assert_eq!(digital_small.template, "digital_receipt");

        let physical_large = plan(&Event::Purchase {
            email: "d@x.test".into(),
            total_cents: 15_000,
            digital: false,
        });
        assert_eq!(physical_large.channel, Channel::Email);
        assert_eq!(physical_large.priority, 3);
        assert_eq!(physical_large.template, "vip_receipt");
    }

    #[test]
    fn password_reset_is_always_email_and_urgent_is_high_priority() {
        let urgent = plan(&Event::PasswordReset { email: "e@x.test".into(), urgent: true });
        assert_eq!(urgent.channel, Channel::Email);
        assert_eq!(urgent.priority, 3);
        assert_eq!(urgent.template, "reset_urgent");

        let normal = plan(&Event::PasswordReset { email: "f@x.test".into(), urgent: false });
        assert_eq!(normal.channel, Channel::Email);
        assert_eq!(normal.priority, 2);
        assert_eq!(normal.template, "reset");
    }

    #[test]
    fn bounce_suppresses_delivery() {
        let bounced = plan(&Event::Bounce { email: "g@x.test".into() });
        assert_eq!(bounced.channel, Channel::None);
        assert_eq!(bounced.priority, 0);
        assert_eq!(bounced.template, "suppress");
    }
}
