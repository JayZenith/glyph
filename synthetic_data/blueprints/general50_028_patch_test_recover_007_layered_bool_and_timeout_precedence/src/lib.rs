#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub endpoint: String,
    pub retries: u8,
    pub timeout_ms: u64,
    pub verbose: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub endpoint: Option<String>,
    pub retries: Option<u8>,
    pub timeout_ms: Option<u64>,
    pub verbose: Option<bool>,
}

pub fn merge_config(defaults: &Config, file: &PartialConfig, cli: &PartialConfig) -> Config {
    Config {
        endpoint: file
            .endpoint
            .clone()
            .or_else(|| cli.endpoint.clone())
            .unwrap_or_else(|| defaults.endpoint.clone()),
        retries: file.retries.or(cli.retries).unwrap_or(defaults.retries),
        timeout_ms: if file.timeout_ms.is_some() {
            file.timeout_ms.unwrap_or(defaults.timeout_ms)
        } else {
            cli.timeout_ms.unwrap_or(defaults.timeout_ms)
        },
        verbose: cli.verbose.unwrap_or(file.verbose.unwrap_or(defaults.verbose)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            endpoint: "https://default.service".to_string(),
            retries: 3,
            timeout_ms: 1000,
            verbose: false,
        }
    }

    #[test]
    fn cli_overrides_file_for_endpoint_and_retries() {
        let file = PartialConfig {
            endpoint: Some("https://file.service".to_string()),
            retries: Some(5),
            timeout_ms: None,
            verbose: None,
        };
        let cli = PartialConfig {
            endpoint: Some("https://cli.service".to_string()),
            retries: Some(1),
            timeout_ms: None,
            verbose: None,
        };

        let merged = merge_config(&defaults(), &file, &cli);
        assert_eq!(merged.endpoint, "https://cli.service");
        assert_eq!(merged.retries, 1);
    }

    #[test]
    fn file_used_when_cli_missing_and_defaults_fill_rest() {
        let file = PartialConfig {
            endpoint: Some("https://file.service".to_string()),
            retries: None,
            timeout_ms: Some(2500),
            verbose: Some(true),
        };
        let cli = PartialConfig::default();

        let merged = merge_config(&defaults(), &file, &cli);
        assert_eq!(merged.endpoint, "https://file.service");
        assert_eq!(merged.retries, 3);
        assert_eq!(merged.timeout_ms, 2500);
        assert!(merged.verbose);
    }

    #[test]
    fn explicit_false_from_cli_must_override_true_from_file() {
        let file = PartialConfig {
            endpoint: None,
            retries: None,
            timeout_ms: None,
            verbose: Some(true),
        };
        let cli = PartialConfig {
            endpoint: None,
            retries: None,
            timeout_ms: None,
            verbose: Some(false),
        };

        let merged = merge_config(&defaults(), &file, &cli);
        assert!(!merged.verbose);
    }

    #[test]
    fn zero_timeout_from_cli_is_explicit_override() {
        let file = PartialConfig {
            endpoint: None,
            retries: None,
            timeout_ms: Some(5000),
            verbose: None,
        };
        let cli = PartialConfig {
            endpoint: None,
            retries: None,
            timeout_ms: Some(0),
            verbose: None,
        };

        let merged = merge_config(&defaults(), &file, &cli);
        assert_eq!(merged.timeout_ms, 0);
    }
}
