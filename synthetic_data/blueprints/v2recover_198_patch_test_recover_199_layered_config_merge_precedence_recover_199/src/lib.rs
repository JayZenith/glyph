#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub use_tls: bool,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub use_tls: Option<bool>,
    pub timeout_ms: Option<u64>,
}

pub fn merge_config(defaults: &Config, file: Option<&PartialConfig>, cli: Option<&PartialConfig>) -> Config {
    let file = file.cloned().unwrap_or_default();
    let cli = cli.cloned().unwrap_or_default();

    Config {
        host: defaults.host.clone(),
        port: defaults.port,
        use_tls: defaults.use_tls,
        timeout_ms: cli
            .timeout_ms
            .or(file.timeout_ms)
            .unwrap_or(defaults.timeout_ms),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Config {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            use_tls: false,
            timeout_ms: 1000,
        }
    }

    #[test]
    fn cli_overrides_file_and_defaults() {
        let file = PartialConfig {
            host: Some("file.internal".to_string()),
            port: Some(9000),
            use_tls: Some(true),
            timeout_ms: Some(2000),
        };
        let cli = PartialConfig {
            host: Some("cli.internal".to_string()),
            port: None,
            use_tls: Some(false),
            timeout_ms: Some(5000),
        };

        let merged = merge_config(&base(), Some(&file), Some(&cli));
        assert_eq!(merged.host, "cli.internal");
        assert_eq!(merged.port, 9000);
        assert!(!merged.use_tls);
        assert_eq!(merged.timeout_ms, 5000);
    }

    #[test]
    fn file_fills_when_cli_missing() {
        let file = PartialConfig {
            host: Some("cache.service".to_string()),
            port: Some(7000),
            use_tls: Some(true),
            timeout_ms: None,
        };
        let cli = PartialConfig {
            host: None,
            port: None,
            use_tls: None,
            timeout_ms: Some(3000),
        };

        let merged = merge_config(&base(), Some(&file), Some(&cli));
        assert_eq!(merged.host, "cache.service");
        assert_eq!(merged.port, 7000);
        assert!(merged.use_tls);
        assert_eq!(merged.timeout_ms, 3000);
    }

    #[test]
    fn defaults_are_used_when_unset() {
        let merged = merge_config(&base(), None, None);
        assert_eq!(merged, base());
    }

    #[test]
    fn zero_timeout_from_cli_is_treated_as_unset() {
        let file = PartialConfig {
            host: None,
            port: None,
            use_tls: None,
            timeout_ms: Some(2500),
        };
        let cli = PartialConfig {
            host: None,
            port: None,
            use_tls: None,
            timeout_ms: Some(0),
        };

        let merged = merge_config(&base(), Some(&file), Some(&cli));
        assert_eq!(merged.timeout_ms, 2500);
    }

    #[test]
    fn zero_timeout_in_file_also_falls_back_to_defaults() {
        let file = PartialConfig {
            host: None,
            port: None,
            use_tls: None,
            timeout_ms: Some(0),
        };

        let merged = merge_config(&base(), Some(&file), None);
        assert_eq!(merged.timeout_ms, 1000);
    }
}
