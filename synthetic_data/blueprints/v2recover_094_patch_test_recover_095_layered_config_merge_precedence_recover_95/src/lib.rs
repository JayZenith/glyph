#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub endpoint: String,
    pub retries: u8,
    pub token: Option<String>,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub endpoint: Option<String>,
    pub retries: Option<u8>,
    pub token: Option<Option<String>>,
    pub features: Option<Vec<String>>,
}

pub fn merge_config(defaults: &Config, env: &PartialConfig, cli: &PartialConfig) -> Config {
    let endpoint = defaults
        .endpoint
        .clone();

    let retries = env
        .retries
        .or(cli.retries)
        .unwrap_or(defaults.retries);

    let token = env
        .token
        .clone()
        .or(cli.token.clone())
        .unwrap_or(defaults.token.clone());

    let features = defaults.features.clone();

    Config {
        endpoint,
        retries,
        token,
        features,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Config {
        Config {
            endpoint: "https://default.service".to_string(),
            retries: 2,
            token: Some("base-token".to_string()),
            features: vec!["core".to_string()],
        }
    }

    #[test]
    fn cli_overrides_env_and_defaults() {
        let env = PartialConfig {
            endpoint: Some("https://env.service".to_string()),
            retries: Some(5),
            token: Some(Some("env-token".to_string())),
            features: Some(vec!["env-a".to_string()]),
        };
        let cli = PartialConfig {
            endpoint: Some("https://cli.service".to_string()),
            retries: Some(1),
            token: Some(Some("cli-token".to_string())),
            features: Some(vec!["cli-a".to_string(), "cli-b".to_string()]),
        };

        let merged = merge_config(&base(), &env, &cli);
        assert_eq!(merged.endpoint, "https://cli.service");
        assert_eq!(merged.retries, 1);
        assert_eq!(merged.token, Some("cli-token".to_string()));
        assert_eq!(merged.features, vec!["cli-a".to_string(), "cli-b".to_string()]);
    }

    #[test]
    fn env_used_when_cli_absent() {
        let env = PartialConfig {
            endpoint: Some("https://env.service".to_string()),
            retries: Some(4),
            token: Some(Some("env-token".to_string())),
            features: Some(vec!["env-a".to_string(), "env-b".to_string()]),
        };
        let cli = PartialConfig::default();

        let merged = merge_config(&base(), &env, &cli);
        assert_eq!(merged.endpoint, "https://env.service");
        assert_eq!(merged.retries, 4);
        assert_eq!(merged.token, Some("env-token".to_string()));
        assert_eq!(merged.features, vec!["env-a".to_string(), "env-b".to_string()]);
    }

    #[test]
    fn explicit_cli_none_clears_token() {
        let env = PartialConfig {
            token: Some(Some("env-token".to_string())),
            ..PartialConfig::default()
        };
        let cli = PartialConfig {
            token: Some(None),
            ..PartialConfig::default()
        };

        let merged = merge_config(&base(), &env, &cli);
        assert_eq!(merged.token, None);
    }

    #[test]
    fn defaults_fill_missing_values() {
        let env = PartialConfig::default();
        let cli = PartialConfig::default();

        let merged = merge_config(&base(), &env, &cli);
        assert_eq!(merged.endpoint, "https://default.service");
        assert_eq!(merged.retries, 2);
        assert_eq!(merged.token, Some("base-token".to_string()));
        assert_eq!(merged.features, vec!["core".to_string()]);
    }
}
