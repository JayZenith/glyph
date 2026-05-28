#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub endpoint: String,
    pub retries: u8,
    pub verbose: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub endpoint: Option<String>,
    pub retries: Option<u8>,
    pub verbose: Option<bool>,
}

pub fn merge_config(defaults: &Config, env: &PartialConfig, cli: &PartialConfig) -> Config {
    Config {
        endpoint: defaults
            .endpoint
            .clone()
            .or_else(|| cli.endpoint.clone())
            .or_else(|| env.endpoint.clone())
            .unwrap_or_default(),
        retries: defaults.retries,
        verbose: cli.verbose.or(env.verbose).unwrap_or(defaults.verbose),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            endpoint: "https://default.service".to_string(),
            retries: 2,
            verbose: false,
        }
    }

    #[test]
    fn env_overrides_defaults_when_cli_missing() {
        let env = PartialConfig {
            endpoint: Some("https://env.service".to_string()),
            retries: Some(5),
            verbose: Some(true),
        };
        let cli = PartialConfig::default();

        let merged = merge_config(&defaults(), &env, &cli);

        assert_eq!(merged.endpoint, "https://env.service");
        assert_eq!(merged.retries, 5);
        assert!(merged.verbose);
    }

    #[test]
    fn cli_overrides_env_for_endpoint_and_retries() {
        let env = PartialConfig {
            endpoint: Some("https://env.service".to_string()),
            retries: Some(4),
            verbose: Some(false),
        };
        let cli = PartialConfig {
            endpoint: Some("https://cli.service".to_string()),
            retries: Some(1),
            verbose: None,
        };

        let merged = merge_config(&defaults(), &env, &cli);

        assert_eq!(merged.endpoint, "https://cli.service");
        assert_eq!(merged.retries, 1);
        assert!(!merged.verbose);
    }

    #[test]
    fn empty_cli_endpoint_does_not_erase_env_value() {
        let env = PartialConfig {
            endpoint: Some("https://env.service".to_string()),
            retries: None,
            verbose: None,
        };
        let cli = PartialConfig {
            endpoint: Some(String::new()),
            retries: None,
            verbose: None,
        };

        let merged = merge_config(&defaults(), &env, &cli);

        assert_eq!(merged.endpoint, "https://env.service");
    }
}
