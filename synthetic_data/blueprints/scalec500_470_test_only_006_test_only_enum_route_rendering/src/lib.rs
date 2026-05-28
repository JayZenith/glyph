#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Delivery {
    Standard,
    Express,
    Pickup { locker: bool },
    Drone { battery_low: bool },
}

pub fn render_delivery(delivery: Delivery) -> String {
    match delivery {
        Delivery::Standard => "standard: 5d".to_string(),
        Delivery::Express => "express: 2d".to_string(),
        Delivery::Pickup { locker: true } => "pickup: locker".to_string(),
        Delivery::Pickup { locker: false } => "pickup: counter".to_string(),
        Delivery::Drone { battery_low: true } => "drone: delayed".to_string(),
        Delivery::Drone { battery_low: false } => "drone: same-day".to_string(),
    }
}

pub fn requires_signature(delivery: &Delivery) -> bool {
    match delivery {
        Delivery::Standard => false,
        Delivery::Express => true,
        Delivery::Pickup { .. } => false,
        Delivery::Drone { battery_low } => !battery_low,
    }
}

pub fn priority_score(delivery: &Delivery) -> u8 {
    match delivery {
        Delivery::Standard => 1,
        Delivery::Express => 3,
        Delivery::Pickup { locker: true } => 2,
        Delivery::Pickup { locker: false } => 1,
        Delivery::Drone { battery_low: true } => 2,
        Delivery::Drone { battery_low: false } => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_every_variant_with_specific_branch_text() {
        assert_eq!(render_delivery(Delivery::Standard), "standard: 5d");
        assert_eq!(render_delivery(Delivery::Express), "express: 2d");
        assert_eq!(
            render_delivery(Delivery::Pickup { locker: true }),
            "pickup: locker"
        );
        assert_eq!(
            render_delivery(Delivery::Pickup { locker: false }),
            "pickup: counter"
        );
        assert_eq!(
            render_delivery(Delivery::Drone { battery_low: true }),
            "drone: delayed"
        );
        assert_eq!(
            render_delivery(Delivery::Drone { battery_low: false }),
            "drone: same-day"
        );
    }

    #[test]
    fn signature_logic_depends_on_variant_and_inner_flag() {
        assert!(!requires_signature(&Delivery::Standard));
        assert!(requires_signature(&Delivery::Express));
        assert!(!requires_signature(&Delivery::Pickup { locker: true }));
        assert!(!requires_signature(&Delivery::Pickup { locker: false }));
        assert!(!requires_signature(&Delivery::Drone { battery_low: true }));
        assert!(requires_signature(&Delivery::Drone { battery_low: false }));
    }

    #[test]
    fn priority_score_distinguishes_similar_enum_branches() {
        let cases = [
            (Delivery::Standard, 1),
            (Delivery::Express, 3),
            (Delivery::Pickup { locker: true }, 2),
            (Delivery::Pickup { locker: false }, 1),
            (Delivery::Drone { battery_low: true }, 2),
            (Delivery::Drone { battery_low: false }, 4),
        ];

        for (delivery, expected) in cases {
            assert_eq!(priority_score(&delivery), expected, "for {:?}", delivery);
        }
    }
}
