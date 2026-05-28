#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub endpoint: Option<String>,
    pub retries: Option<u8>,
    pub use_tls: Option<bool>,
}

impl Config {
    pub fn new(endpoint: Option<&str>, retries: Option<u8>, use_tls: Option<bool>) -> Self {
        Self {
            endpoint: endpoint.map(str::to_string),
            retries,
            use_tls,
        }
    }
}

pub fn merge_config(defaults: &Config, env: &Config, cli: &Config) -> Config {
    Config {
        endpoint: cli
            .endpoint
            .clone()
            .or_else(|| env.endpoint.clone())
            .or_else(|| defaults.endpoint.clone()),
        retries: cli.retries.or(env.retries).or(defaults.retries),
        use_tls: cli.use_tls.filter(|v| *v).or(env.use_tls.filter(|v| *v)).or(defaults.use_tls),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_overrides_env_and_defaults() {
        let defaults = Config::new(Some("https://default.service"), Some(3), Some(true));
        let env = Config::new(Some("https://env.service"), Some(5), Some(true));
        let cli = Config::new(Some("https://cli.service"), Some(1), Some(false));

        let merged = merge_config(&defaults, &env, &cli);

        assert_eq!(merged.endpoint.as_deref(), Some("https://cli.service"));
        assert_eq!(merged.retries, Some(1));
        assert_eq!(merged.use_tls, Some(false));
    }

    #[test]
    fn explicit_false_from_env_beats_default_true() {
        let defaults = Config::new(Some("https://default.service"), Some(3), Some(true));
        let env = Config::new(None, None, Some(false));
        let cli = Config::new(None, None, None);

        let merged = merge_config(&defaults, &env, &cli);

        assert_eq!(merged.endpoint.as_deref(), Some("https://default.service"));
        assert_eq!(merged.retries, Some(3));
        assert_eq!(merged.use_tls, Some(false));
    }

    #[test]
    fn falls_back_when_higher_precedence_layers_are_unset() {
        let defaults = Config::new(Some("https://default.service"), Some(3), Some(true));
        let env = Config::new(None, Some(7), None);
        let cli = Config::new(None, None, None);

        let merged = merge_config(&defaults, &env, &cli);

        assert_eq!(merged.endpoint.as_deref(), Some("https://default.service"));
        assert_eq!(merged.retries, Some(7));
        assert_eq!(merged.use_tls, Some(true));
    }
}
