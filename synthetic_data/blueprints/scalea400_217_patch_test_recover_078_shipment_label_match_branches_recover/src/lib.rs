pub enum Shipment {
    Standard { fragile: bool },
    Express { saturday: bool },
    International { customs_paid: bool, priority: bool },
    Pickup,
}

pub fn label(shipment: Shipment) -> &'static str {
    match shipment {
        Shipment::Standard { .. } => "standard",
        Shipment::Express { .. } => "express",
        Shipment::International { customs_paid, .. } => {
            if customs_paid {
                "intl-cleared"
            } else {
                "intl-hold"
            }
        }
        Shipment::Pickup => "pickup",
    }
}

#[cfg(test)]
mod tests {
    use super::{label, Shipment};

    #[test]
    fn standard_fragile_gets_special_label() {
        assert_eq!(label(Shipment::Standard { fragile: true }), "standard-fragile");
    }

    #[test]
    fn standard_non_fragile_stays_standard() {
        assert_eq!(label(Shipment::Standard { fragile: false }), "standard");
    }

    #[test]
    fn express_saturday_gets_weekend_label() {
        assert_eq!(label(Shipment::Express { saturday: true }), "express-sat");
    }

    #[test]
    fn express_weekday_stays_express() {
        assert_eq!(label(Shipment::Express { saturday: false }), "express");
    }

    #[test]
    fn international_priority_and_cleared_is_expedited() {
        assert_eq!(
            label(Shipment::International {
                customs_paid: true,
                priority: true,
            }),
            "intl-priority"
        );
    }

    #[test]
    fn international_uncleared_is_on_hold_even_if_priority() {
        assert_eq!(
            label(Shipment::International {
                customs_paid: false,
                priority: true,
            }),
            "intl-hold"
        );
    }

    #[test]
    fn pickup_is_pickup() {
        assert_eq!(label(Shipment::Pickup), "pickup");
    }
}
