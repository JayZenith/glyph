#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub endpoint: Option<String>,
    pub retries: Option<u8>,
    pub timeout_ms: Option<u64>,
    pub verbose: Option<bool>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            endpoint: None,
            retries: None,
            timeout_ms: None,
            verbose: None,
        }
    }
}

pub fn merge_configs(defaults: &Config, env: &Config, cli: &Config) -> Config {
    Config {
        endpoint: cli
            .endpoint
            .as_ref()
            .filter(|s| !s.is_empty())
            .cloned()
            .or_else(|| env.endpoint.clone())
            .or_else(|| defaults.endpoint.clone()),
        retries: cli.retries.or(defaults.retries).or(env.retries),
        timeout_ms: cli.timeout_ms.or(env.timeout_ms).or(defaults.timeout_ms),
        verbose: cli.verbose.or(env.verbose).or(defaults.verbose),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(endpoint: Option<&str>, retries: Option<u8>, timeout_ms: Option<u64>, verbose: Option<bool>) -> Config {
        Config {
            endpoint: endpoint.map(|s| s.to_string()),
            retries,
            timeout_ms,
            verbose,
        }
    }

    #[test]
    fn defaults_fill_missing_values() {
        let defaults = cfg(Some("https://default"), Some(3), Some(1000), Some(false));
        let env = cfg(None, None, None, None);
        let cli = cfg(None, None, None, None);

        let merged = merge_configs(&defaults, &env, &cli);
        assert_eq!(merged, defaults);
    }

    #[test]
    fn env_overrides_defaults_when_cli_missing() {
        let defaults = cfg(Some("https://default"), Some(3), Some(1000), Some(false));
        let env = cfg(Some("https://env"), Some(5), Some(2000), Some(true));
        let cli = cfg(None, None, None, None);

        let merged = merge_configs(&defaults, &env, &cli);
        assert_eq!(merged, env);
    }

    #[test]
    fn cli_has_highest_precedence() {
        let defaults = cfg(Some("https://default"), Some(3), Some(1000), Some(false));
        let env = cfg(Some("https://env"), Some(5), Some(2000), Some(false));
        let cli = cfg(Some("https://cli"), Some(7), Some(3000), Some(true));

        let merged = merge_configs(&defaults, &env, &cli);
        assert_eq!(merged, cli);
    }

    #[test]
    fn explicit_empty_cli_endpoint_disables_endpoint() {
        let defaults = cfg(Some("https://default"), Some(3), Some(1000), Some(false));
        let env = cfg(Some("https://env"), Some(5), Some(2000), Some(true));
        let cli = cfg(Some(""), None, None, None);

        let merged = merge_configs(&defaults, &env, &cli);
        assert_eq!(merged.endpoint, Some(String::new()));
        assert_eq!(merged.retries, Some(5));
        assert_eq!(merged.timeout_ms, Some(2000));
        assert_eq!(merged.verbose, Some(true));
    }
}
