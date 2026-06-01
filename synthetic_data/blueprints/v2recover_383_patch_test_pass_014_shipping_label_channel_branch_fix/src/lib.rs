#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delivery {
    Digital,
    Courier { express: bool },
    Pickup { locker: bool },
}

pub fn label_for(delivery: Delivery) -> &'static str {
    match delivery {
        Delivery::Digital => "email",
        Delivery::Courier { express } => {
            if express {
                "courier"
            } else {
                "courier-express"
            }
        }
        Delivery::Pickup { locker } => {
            if locker {
                "store-pickup"
            } else {
                "locker-pickup"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digital_uses_email_channel() {
        assert_eq!(label_for(Delivery::Digital), "email");
    }

    #[test]
    fn courier_variants_are_distinguished() {
        assert_eq!(label_for(Delivery::Courier { express: true }), "courier-express");
        assert_eq!(label_for(Delivery::Courier { express: false }), "courier");
    }

    #[test]
    fn pickup_variants_are_distinguished() {
        assert_eq!(label_for(Delivery::Pickup { locker: true }), "locker-pickup");
        assert_eq!(label_for(Delivery::Pickup { locker: false }), "store-pickup");
    }
}
