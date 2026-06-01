#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub endpoint: String,
    pub region: Option<String>,
    pub retries: u8,
    pub verbose: bool,
}

impl Config {
    pub fn merge(base: &Config, override_cfg: &Config) -> Config {
        Config {
            endpoint: if base.endpoint.is_empty() {
                override_cfg.endpoint.clone()
            } else {
                base.endpoint.clone()
            },
            region: override_cfg.region.clone().or_else(|| base.region.clone()),
            retries: if override_cfg.retries == 0 {
                base.retries
            } else {
                override_cfg.retries
            },
            verbose: base.verbose || override_cfg.verbose,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(endpoint: &str, region: Option<&str>, retries: u8, verbose: bool) -> Config {
        Config {
            endpoint: endpoint.to_string(),
            region: region.map(str::to_string),
            retries,
            verbose,
        }
    }

    #[test]
    fn override_values_take_precedence() {
        let base = cfg("https://base.example", Some("us-east-1"), 3, false);
        let override_cfg = cfg("https://override.example", Some("eu-west-1"), 5, true);

        let merged = Config::merge(&base, &override_cfg);

        assert_eq!(merged.endpoint, "https://override.example");
        assert_eq!(merged.region, Some("eu-west-1".to_string()));
        assert_eq!(merged.retries, 5);
        assert!(merged.verbose);
    }

    #[test]
    fn empty_override_endpoint_keeps_base_endpoint() {
        let base = cfg("https://base.example", Some("us-east-1"), 2, false);
        let override_cfg = cfg("", None, 0, true);

        let merged = Config::merge(&base, &override_cfg);

        assert_eq!(merged.endpoint, "https://base.example");
        assert_eq!(merged.region, Some("us-east-1".to_string()));
        assert_eq!(merged.retries, 2);
        assert!(merged.verbose);
    }

    #[test]
    fn base_region_is_used_when_override_region_missing() {
        let base = cfg("https://base.example", Some("ap-south-1"), 4, false);
        let override_cfg = cfg("https://override.example", None, 0, false);

        let merged = Config::merge(&base, &override_cfg);

        assert_eq!(merged.region, Some("ap-south-1".to_string()));
        assert_eq!(merged.endpoint, "https://override.example");
        assert_eq!(merged.retries, 4);
        assert!(!merged.verbose);
    }
}
