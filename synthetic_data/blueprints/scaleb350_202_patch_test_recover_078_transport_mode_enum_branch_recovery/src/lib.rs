#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transport {
    Car { electric: bool },
    Bike,
    Train { express: bool },
    Bus,
}

pub fn travel_label(mode: Transport) -> &'static str {
    match mode {
        Transport::Car { electric: true } => "road",
        Transport::Car { electric: false } => "road",
        Transport::Bike => "road",
        Transport::Train { express: true } => "rail-local",
        Transport::Train { express: false } => "rail-express",
        Transport::Bus => "rail",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cars_distinguish_powertrain() {
        assert_eq!(travel_label(Transport::Car { electric: true }), "road-electric");
        assert_eq!(travel_label(Transport::Car { electric: false }), "road-fuel");
    }

    #[test]
    fn train_branch_matches_express_flag() {
        assert_eq!(travel_label(Transport::Train { express: true }), "rail-express");
        assert_eq!(travel_label(Transport::Train { express: false }), "rail-local");
    }

    #[test]
    fn bike_and_bus_have_fixed_labels() {
        assert_eq!(travel_label(Transport::Bike), "road-bike");
        assert_eq!(travel_label(Transport::Bus), "road-bus");
    }
}
