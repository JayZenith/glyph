#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub endpoint: Option<String>,
    pub timeout_ms: Option<u64>,
    pub retries: Option<u8>,
    pub features: Vec<String>,
}

impl Config {
    pub fn new(
        endpoint: Option<&str>,
        timeout_ms: Option<u64>,
        retries: Option<u8>,
        features: &[&str],
    ) -> Self {
        Self {
            endpoint: endpoint.map(str::to_string),
            timeout_ms,
            retries,
            features: features.iter().map(|s| s.to_string()).collect(),
        }
    }
}

pub fn merge_config(base: &Config, env: &Config, cli: &Config) -> Config {
    let endpoint = base
        .endpoint
        .clone()
        .or_else(|| env.endpoint.clone())
        .or_else(|| cli.endpoint.clone());

    let timeout_ms = base.timeout_ms.or(env.timeout_ms).or(cli.timeout_ms);
    let retries = base.retries.or(env.retries).or(cli.retries);

    let mut features = base.features.clone();
    if !env.features.is_empty() {
        features = env.features.clone();
    }
    if !cli.features.is_empty() {
        features = cli.features.clone();
    }

    Config {
        endpoint,
        timeout_ms,
        retries,
        features,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_has_highest_precedence_for_scalars() {
        let base = Config::new(Some("https://base"), Some(1000), Some(1), &["base-a"]);
        let env = Config::new(Some("https://env"), Some(2000), Some(2), &["env-a"]);
        let cli = Config::new(Some("https://cli"), Some(3000), Some(3), &["cli-a"]);

        let merged = merge_config(&base, &env, &cli);

        assert_eq!(merged.endpoint, Some("https://cli".to_string()));
        assert_eq!(merged.timeout_ms, Some(3000));
        assert_eq!(merged.retries, Some(3));
    }

    #[test]
    fn env_used_when_cli_missing_and_empty_endpoint_is_intentional() {
        let base = Config::new(Some("https://base"), Some(1000), Some(1), &["base-a"]);
        let env = Config::new(Some(""), Some(2500), Some(4), &["env-a"]);
        let cli = Config::new(None, None, None, &[]);

        let merged = merge_config(&base, &env, &cli);

        assert_eq!(merged.endpoint, Some("".to_string()));
        assert_eq!(merged.timeout_ms, Some(2500));
        assert_eq!(merged.retries, Some(4));
    }

    #[test]
    fn features_append_in_layer_order_without_dedup() {
        let base = Config::new(Some("https://base"), Some(1000), Some(1), &["a", "b"]);
        let env = Config::new(None, None, None, &["b", "c"]);
        let cli = Config::new(None, None, None, &["d"]);

        let merged = merge_config(&base, &env, &cli);

        assert_eq!(merged.features, vec!["a", "b", "b", "c", "d"]);
    }

    #[test]
    fn empty_feature_layers_do_not_clear_previous_values() {
        let base = Config::new(None, None, None, &["base-a"]);
        let env = Config::new(None, None, None, &[]);
        let cli = Config::new(None, None, None, &[]);

        let merged = merge_config(&base, &env, &cli);

        assert_eq!(merged.features, vec!["base-a"]);
    }
}
