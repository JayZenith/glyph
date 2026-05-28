#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Carrier {
    Postal,
    Express,
    Freight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Speed {
    Economy,
    Standard,
    Priority,
}

pub fn service_code(carrier: Carrier, speed: Speed, international: bool) -> &'static str {
    match carrier {
        Carrier::Postal => match speed {
            Speed::Economy => {
                if international { "PIE" } else { "PDE" }
            }
            Speed::Standard => {
                if international { "PIS" } else { "PDS" }
            }
            Speed::Priority => {
                if international { "PIP" } else { "PDP" }
            }
        },
        Carrier::Express => match speed {
            Speed::Economy => {
                if international { "XIE" } else { "XDE" }
            }
            Speed::Standard => {
                if international { "XIS" } else { "XDS" }
            }
            Speed::Priority => {
                if international { "XIS" } else { "XDP" }
            }
        },
        Carrier::Freight => {
            if international {
                "FRT-INT"
            } else {
                match speed {
                    Speed::Economy => "FRT-ECO",
                    Speed::Standard => "FRT-STD",
                    Speed::Priority => "FRT-PRI",
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn postal_codes_cover_all_speeds() {
        assert_eq!(service_code(Carrier::Postal, Speed::Economy, false), "PDE");
        assert_eq!(service_code(Carrier::Postal, Speed::Standard, true), "PIS");
        assert_eq!(service_code(Carrier::Postal, Speed::Priority, false), "PDP");
    }

    #[test]
    fn express_priority_uses_distinct_domestic_and_international_codes() {
        assert_eq!(service_code(Carrier::Express, Speed::Priority, false), "XDP");
        assert_eq!(service_code(Carrier::Express, Speed::Priority, true), "XIP");
    }

    #[test]
    fn freight_ignores_speed_when_international() {
        assert_eq!(service_code(Carrier::Freight, Speed::Economy, true), "FRT-INT");
        assert_eq!(service_code(Carrier::Freight, Speed::Priority, true), "FRT-INT");
    }
}
