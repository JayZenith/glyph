#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub endpoint: Option<String>,
    pub retries: u8,
    pub timeout_ms: u64,
    pub use_tls: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            endpoint: Some("https://default.service".to_string()),
            retries: 3,
            timeout_ms: 1000,
            use_tls: true,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub endpoint: Option<String>,
    pub retries: Option<u8>,
    pub timeout_ms: Option<u64>,
    pub use_tls: Option<bool>,
}

pub fn merge_config(file: PartialConfig, env: PartialConfig, cli: PartialConfig) -> Config {
    let mut merged = Config::default();

    if let Some(endpoint) = file.endpoint {
        merged.endpoint = Some(endpoint);
    }
    if let Some(retries) = file.retries {
        merged.retries = retries;
    }
    if let Some(timeout_ms) = file.timeout_ms {
        merged.timeout_ms = timeout_ms;
    }
    if let Some(use_tls) = file.use_tls {
        merged.use_tls = use_tls;
    }

    if let Some(endpoint) = env.endpoint {
        if !endpoint.is_empty() {
            merged.endpoint = Some(endpoint);
        }
    }
    if let Some(retries) = env.retries {
        if retries > 0 {
            merged.retries = retries;
        }
    }
    if let Some(timeout_ms) = env.timeout_ms {
        merged.timeout_ms = timeout_ms;
    }
    if let Some(use_tls) = env.use_tls {
        merged.use_tls = use_tls;
    }

    if let Some(endpoint) = cli.endpoint {
        merged.endpoint = Some(endpoint);
    }
    if let Some(retries) = cli.retries {
        merged.retries = retries;
    }
    if let Some(timeout_ms) = cli.timeout_ms {
        merged.timeout_ms = timeout_ms;
    }
    if let Some(use_tls) = cli.use_tls {
        merged.use_tls = use_tls;
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_overrides_defaults_when_env_and_cli_absent() {
        let file = PartialConfig {
            endpoint: Some("https://file.service".into()),
            retries: Some(5),
            timeout_ms: Some(2500),
            use_tls: Some(false),
        };

        let merged = merge_config(file, PartialConfig::default(), PartialConfig::default());

        assert_eq!(
            merged,
            Config {
                endpoint: Some("https://file.service".into()),
                retries: 5,
                timeout_ms: 2500,
                use_tls: false,
            }
        );
    }

    #[test]
    fn env_overrides_file_and_can_clear_endpoint_with_empty_string() {
        let file = PartialConfig {
            endpoint: Some("https://file.service".into()),
            retries: Some(4),
            timeout_ms: Some(1500),
            use_tls: Some(true),
        };
        let env = PartialConfig {
            endpoint: Some(String::new()),
            retries: Some(0),
            timeout_ms: Some(900),
            use_tls: Some(false),
        };

        let merged = merge_config(file, env, PartialConfig::default());

        assert_eq!(merged.endpoint, None);
        assert_eq!(merged.retries, 0);
        assert_eq!(merged.timeout_ms, 900);
        assert!(!merged.use_tls);
    }

    #[test]
    fn cli_has_highest_precedence() {
        let file = PartialConfig {
            endpoint: Some("https://file.service".into()),
            retries: Some(2),
            timeout_ms: Some(1200),
            use_tls: Some(false),
        };
        let env = PartialConfig {
            endpoint: Some("https://env.service".into()),
            retries: Some(4),
            timeout_ms: Some(1600),
            use_tls: Some(true),
        };
        let cli = PartialConfig {
            endpoint: Some("https://cli.service".into()),
            retries: Some(7),
            timeout_ms: Some(3000),
            use_tls: Some(false),
        };

        let merged = merge_config(file, env, cli);

        assert_eq!(merged.endpoint, Some("https://cli.service".into()));
        assert_eq!(merged.retries, 7);
        assert_eq!(merged.timeout_ms, 3000);
        assert!(!merged.use_tls);
    }
}
