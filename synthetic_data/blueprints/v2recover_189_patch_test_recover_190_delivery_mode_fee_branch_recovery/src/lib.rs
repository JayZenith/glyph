#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delivery {
    Standard,
    Express,
    Pickup,
}

pub fn delivery_fee(mode: Delivery, fragile: bool, distance_km: u32) -> u32 {
    match mode {
        Delivery::Standard => {
            if distance_km <= 5 {
                3
            } else {
                6
            }
        }
        Delivery::Express => {
            if distance_km <= 10 {
                10
            } else {
                10 + (distance_km - 10) / 5
            }
        }
        Delivery::Pickup => {
            if fragile {
                2
            } else {
                1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{delivery_fee, Delivery};

    #[test]
    fn standard_adds_fragile_fee_only_for_non_pickup() {
        assert_eq!(delivery_fee(Delivery::Standard, false, 4), 3);
        assert_eq!(delivery_fee(Delivery::Standard, true, 4), 5);
        assert_eq!(delivery_fee(Delivery::Standard, true, 8), 8);
    }

    #[test]
    fn express_uses_rounded_up_distance_blocks_and_fragile_fee() {
        assert_eq!(delivery_fee(Delivery::Express, false, 10), 10);
        assert_eq!(delivery_fee(Delivery::Express, false, 11), 11);
        assert_eq!(delivery_fee(Delivery::Express, false, 15), 11);
        assert_eq!(delivery_fee(Delivery::Express, false, 16), 12);
        assert_eq!(delivery_fee(Delivery::Express, true, 16), 14);
    }

    #[test]
    fn pickup_is_free_unless_fragile() {
        assert_eq!(delivery_fee(Delivery::Pickup, false, 1), 0);
        assert_eq!(delivery_fee(Delivery::Pickup, false, 99), 0);
        assert_eq!(delivery_fee(Delivery::Pickup, true, 99), 2);
    }
}
