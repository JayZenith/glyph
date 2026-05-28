#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub timeout_ms: u32,
    pub endpoint: String,
    pub retries: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConfigPatch {
    pub timeout_ms: Option<u32>,
    pub endpoint: Option<String>,
    pub retries: Option<Option<u8>>,
}

pub fn merge_config(base: &Config, env: &ConfigPatch, cli: &ConfigPatch) -> Config {
    let timeout_ms = cli
        .timeout_ms
        .or(env.timeout_ms)
        .unwrap_or(base.timeout_ms);

    let endpoint = cli
        .endpoint
        .as_ref()
        .or(env.endpoint.as_ref())
        .cloned()
        .unwrap_or_else(|| base.endpoint.clone());

    let retries = cli
        .retries
        .flatten()
        .or(env.retries.flatten())
        .unwrap_or(base.retries);

    Config {
        timeout_ms,
        endpoint,
        retries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Config {
        Config {
            timeout_ms: 1000,
            endpoint: "https://default.service".to_string(),
            retries: 3,
        }
    }

    #[test]
    fn cli_overrides_env_and_base_for_plain_values() {
        let env = ConfigPatch {
            timeout_ms: Some(2000),
            endpoint: Some("https://env.service".to_string()),
            retries: Some(Some(4)),
        };
        let cli = ConfigPatch {
            timeout_ms: Some(5000),
            endpoint: Some("https://cli.service".to_string()),
            retries: Some(Some(1)),
        };

        let merged = merge_config(&base(), &env, &cli);
        assert_eq!(
            merged,
            Config {
                timeout_ms: 5000,
                endpoint: "https://cli.service".to_string(),
                retries: 1,
            }
        );
    }

    #[test]
    fn env_applies_when_cli_does_not_set_value() {
        let env = ConfigPatch {
            timeout_ms: Some(2500),
            endpoint: None,
            retries: Some(Some(5)),
        };
        let cli = ConfigPatch::default();

        let merged = merge_config(&base(), &env, &cli);
        assert_eq!(merged.timeout_ms, 2500);
        assert_eq!(merged.endpoint, "https://default.service");
        assert_eq!(merged.retries, 5);
    }

    #[test]
    fn explicit_none_from_cli_disables_retries_even_if_env_sets_it() {
        let env = ConfigPatch {
            timeout_ms: None,
            endpoint: None,
            retries: Some(Some(7)),
        };
        let cli = ConfigPatch {
            timeout_ms: None,
            endpoint: None,
            retries: Some(None),
        };

        let merged = merge_config(&base(), &env, &cli);
        assert_eq!(merged.retries, 0);
    }

    #[test]
    fn explicit_none_from_env_disables_retries_when_cli_is_unset() {
        let env = ConfigPatch {
            timeout_ms: None,
            endpoint: None,
            retries: Some(None),
        };
        let cli = ConfigPatch::default();

        let merged = merge_config(&base(), &env, &cli);
        assert_eq!(merged.retries, 0);
    }
}
