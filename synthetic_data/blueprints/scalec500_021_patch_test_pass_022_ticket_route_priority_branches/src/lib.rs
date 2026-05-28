#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ticket {
    Incident { urgent: bool, customer_tier: u8 },
    Question { has_sla: bool },
    Billing { disputed: bool, amount_cents: u32 },
}

pub fn route(ticket: Ticket) -> &'static str {
    match ticket {
        Ticket::Incident { urgent, .. } => {
            if urgent {
                "pager"
            } else {
                "support"
            }
        }
        Ticket::Question { has_sla } => {
            if has_sla {
                "support"
            } else {
                "queue"
            }
        }
        Ticket::Billing { disputed, amount_cents } => {
            if disputed {
                "billing"
            } else if amount_cents >= 50_000 {
                "finance"
            } else {
                "queue"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{route, Ticket};

    #[test]
    fn urgent_incident_always_pages() {
        assert_eq!(
            route(Ticket::Incident {
                urgent: true,
                customer_tier: 1,
            }),
            "pager"
        );
    }

    #[test]
    fn vip_incident_pages_even_when_not_urgent() {
        assert_eq!(
            route(Ticket::Incident {
                urgent: false,
                customer_tier: 3,
            }),
            "pager"
        );
    }

    #[test]
    fn regular_nonurgent_incident_goes_to_support() {
        assert_eq!(
            route(Ticket::Incident {
                urgent: false,
                customer_tier: 2,
            }),
            "support"
        );
    }

    #[test]
    fn sla_question_goes_to_support() {
        assert_eq!(route(Ticket::Question { has_sla: true }), "support");
    }

    #[test]
    fn disputed_billing_always_goes_to_billing() {
        assert_eq!(
            route(Ticket::Billing {
                disputed: true,
                amount_cents: 120_000,
            }),
            "billing"
        );
    }

    #[test]
    fn large_clean_billing_goes_to_finance() {
        assert_eq!(
            route(Ticket::Billing {
                disputed: false,
                amount_cents: 75_000,
            }),
            "finance"
        );
    }
}
