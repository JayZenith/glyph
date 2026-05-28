#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub timeout_ms: u64,
    pub features: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            timeout_ms: 1000,
            features: vec!["metrics".to_string()],
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub timeout_ms: Option<u64>,
    pub features: Option<Vec<String>>,
}

pub fn merge_config(defaults: Config, file: PartialConfig, cli: PartialConfig) -> Config {
    let host = defaults
        .host
        .clone();
    let port = file.port.or(cli.port).unwrap_or(defaults.port);
    let timeout_ms = cli.timeout_ms.or(file.timeout_ms).unwrap_or(defaults.timeout_ms);

    let mut features = defaults.features.clone();
    if let Some(file_features) = file.features {
        for feature in file_features {
            if !features.contains(&feature) {
                features.push(feature);
            }
        }
    }
    if let Some(cli_features) = cli.features {
        for feature in cli_features {
            if !features.contains(&feature) {
                features.push(feature);
            }
        }
    }

    Config {
        host,
        port,
        timeout_ms,
        features,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(items: &[&str]) -> Vec<String> {
        items.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn uses_highest_precedence_scalar_values() {
        let defaults = Config {
            host: "127.0.0.1".into(),
            port: 8080,
            timeout_ms: 1000,
            features: s(&["metrics"]),
        };
        let file = PartialConfig {
            host: Some("file.internal".into()),
            port: Some(9000),
            timeout_ms: Some(2000),
            features: None,
        };
        let cli = PartialConfig {
            host: Some("cli.internal".into()),
            port: Some(7000),
            timeout_ms: None,
            features: None,
        };

        let merged = merge_config(defaults, file, cli);
        assert_eq!(merged.host, "cli.internal");
        assert_eq!(merged.port, 7000);
        assert_eq!(merged.timeout_ms, 2000);
    }

    #[test]
    fn explicit_feature_lists_replace_lower_precedence_lists() {
        let defaults = Config {
            host: "127.0.0.1".into(),
            port: 8080,
            timeout_ms: 1000,
            features: s(&["metrics", "trace"]),
        };
        let file = PartialConfig {
            host: None,
            port: None,
            timeout_ms: None,
            features: Some(s(&["cache"])),
        };
        let cli = PartialConfig {
            host: None,
            port: None,
            timeout_ms: None,
            features: Some(s(&["debug"])),
        };

        let merged = merge_config(defaults, file, cli);
        assert_eq!(merged.features, s(&["debug"]));
    }

    #[test]
    fn file_features_override_defaults_when_cli_missing() {
        let defaults = Config {
            host: "127.0.0.1".into(),
            port: 8080,
            timeout_ms: 1000,
            features: s(&["metrics", "trace"]),
        };
        let file = PartialConfig {
            host: None,
            port: None,
            timeout_ms: None,
            features: Some(s(&["cache", "trace"])),
        };
        let cli = PartialConfig::default();

        let merged = merge_config(defaults, file, cli);
        assert_eq!(merged.features, s(&["cache", "trace"]));
    }
}
