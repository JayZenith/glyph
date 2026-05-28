#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub endpoint: String,
    pub retries: u8,
    pub cache_enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub endpoint: Option<String>,
    pub retries: Option<u8>,
    pub cache_enabled: Option<bool>,
}

pub fn merge_config(defaults: Config, env: PartialConfig, cli: PartialConfig) -> Config {
    Config {
        endpoint: cli
            .endpoint
            .or(env.endpoint)
            .unwrap_or(defaults.endpoint),
        retries: cli.retries.or(env.retries).unwrap_or(defaults.retries),
        cache_enabled: cli
            .cache_enabled
            .or_else(|| env.cache_enabled)
            .unwrap_or(defaults.cache_enabled || env.cache_enabled.unwrap_or(false)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            endpoint: "https://default.local".to_string(),
            retries: 2,
            cache_enabled: false,
        }
    }

    #[test]
    fn cli_overrides_env_for_endpoint_and_retries() {
        let env = PartialConfig {
            endpoint: Some("https://env.local".to_string()),
            retries: Some(5),
            cache_enabled: None,
        };
        let cli = PartialConfig {
            endpoint: Some("https://cli.local".to_string()),
            retries: Some(1),
            cache_enabled: None,
        };

        let merged = merge_config(defaults(), env, cli);
        assert_eq!(merged.endpoint, "https://cli.local");
        assert_eq!(merged.retries, 1);
        assert!(!merged.cache_enabled);
    }

    #[test]
    fn env_fills_missing_values() {
        let env = PartialConfig {
            endpoint: Some("https://env.local".to_string()),
            retries: Some(4),
            cache_enabled: Some(true),
        };
        let cli = PartialConfig::default();

        let merged = merge_config(defaults(), env, cli);
        assert_eq!(merged.endpoint, "https://env.local");
        assert_eq!(merged.retries, 4);
        assert!(merged.cache_enabled);
    }

    #[test]
    fn explicit_cli_disable_beats_env_enable() {
        let env = PartialConfig {
            endpoint: None,
            retries: None,
            cache_enabled: Some(true),
        };
        let cli = PartialConfig {
            endpoint: None,
            retries: None,
            cache_enabled: Some(false),
        };

        let merged = merge_config(defaults(), env, cli);
        assert!(!merged.cache_enabled);
    }

    #[test]
    fn defaults_are_used_when_no_overrides_exist() {
        let merged = merge_config(defaults(), PartialConfig::default(), PartialConfig::default());
        assert_eq!(merged.endpoint, "https://default.local");
        assert_eq!(merged.retries, 2);
        assert!(!merged.cache_enabled);
    }
}
