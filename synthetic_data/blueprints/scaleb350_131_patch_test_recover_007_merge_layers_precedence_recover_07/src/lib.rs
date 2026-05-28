#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub endpoint: String,
    pub retries: u8,
    pub use_tls: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub endpoint: Option<String>,
    pub retries: Option<u8>,
    pub use_tls: Option<bool>,
}

pub fn merge_config(
    defaults: &Config,
    profile: Option<&PartialConfig>,
    env: Option<&PartialConfig>,
    cli: Option<&PartialConfig>,
) -> Config {
    let endpoint = cli
        .and_then(|c| c.endpoint.clone())
        .or_else(|| env.and_then(|c| c.endpoint.clone()))
        .or_else(|| profile.and_then(|c| c.endpoint.clone()))
        .unwrap_or_else(|| defaults.endpoint.clone());

    let retries = cli
        .and_then(|c| c.retries)
        .or_else(|| profile.and_then(|c| c.retries))
        .or_else(|| env.and_then(|c| c.retries))
        .unwrap_or(defaults.retries);

    let use_tls = cli
        .and_then(|c| c.use_tls)
        .or_else(|| env.and_then(|c| c.use_tls))
        .unwrap_or(defaults.use_tls);

    Config {
        endpoint,
        retries,
        use_tls,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Config {
        Config {
            endpoint: "https://default.service".to_string(),
            retries: 2,
            use_tls: true,
        }
    }

    #[test]
    fn profile_applies_when_no_higher_layer_sets_value() {
        let profile = PartialConfig {
            endpoint: Some("https://profile.service".to_string()),
            retries: Some(4),
            use_tls: Some(false),
        };

        let merged = merge_config(&base(), Some(&profile), None, None);
        assert_eq!(
            merged,
            Config {
                endpoint: "https://profile.service".to_string(),
                retries: 4,
                use_tls: false,
            }
        );
    }

    #[test]
    fn env_should_override_profile_for_numeric_settings() {
        let profile = PartialConfig {
            endpoint: None,
            retries: Some(4),
            use_tls: None,
        };
        let env = PartialConfig {
            endpoint: None,
            retries: Some(7),
            use_tls: None,
        };

        let merged = merge_config(&base(), Some(&profile), Some(&env), None);
        assert_eq!(merged.retries, 7);
    }

    #[test]
    fn cli_false_must_override_default_true() {
        let cli = PartialConfig {
            endpoint: None,
            retries: None,
            use_tls: Some(false),
        };

        let merged = merge_config(&base(), None, None, Some(&cli));
        assert!(!merged.use_tls);
    }

    #[test]
    fn env_false_must_override_profile_true() {
        let profile = PartialConfig {
            endpoint: None,
            retries: None,
            use_tls: Some(true),
        };
        let env = PartialConfig {
            endpoint: None,
            retries: None,
            use_tls: Some(false),
        };

        let merged = merge_config(&base(), Some(&profile), Some(&env), None);
        assert!(!merged.use_tls);
    }

    #[test]
    fn endpoint_precedence_is_cli_then_env_then_profile_then_default() {
        let profile = PartialConfig {
            endpoint: Some("https://profile.service".to_string()),
            retries: None,
            use_tls: None,
        };
        let env = PartialConfig {
            endpoint: Some("https://env.service".to_string()),
            retries: None,
            use_tls: None,
        };
        let cli = PartialConfig {
            endpoint: Some("https://cli.service".to_string()),
            retries: None,
            use_tls: None,
        };

        let merged = merge_config(&base(), Some(&profile), Some(&env), Some(&cli));
        assert_eq!(merged.endpoint, "https://cli.service");
    }
}
