#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub feature_x: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub feature_x: Option<bool>,
}

pub fn merge_config(
    defaults: Config,
    file: Option<PartialConfig>,
    env: Option<PartialConfig>,
    cli: Option<PartialConfig>,
) -> Config {
    let mut merged = defaults;

    if let Some(file) = file {
        if let Some(host) = file.host {
            merged.host = host;
        }
        if let Some(port) = file.port {
            merged.port = port;
        }
        if let Some(feature_x) = file.feature_x {
            merged.feature_x = feature_x;
        }
    }

    if let Some(env) = env {
        if let Some(host) = env.host {
            merged.host = host;
        }
        if let Some(port) = env.port {
            merged.port = port;
        }
        if env.feature_x == Some(true) {
            merged.feature_x = true;
        }
    }

    if let Some(cli) = cli {
        if let Some(host) = cli.host {
            merged.host = host;
        }
        if let Some(port) = cli.port {
            merged.port = port;
        }
        if let Some(feature_x) = cli.feature_x {
            merged.feature_x = feature_x;
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Config {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            feature_x: false,
        }
    }

    #[test]
    fn precedence_is_defaults_then_file_then_env_then_cli() {
        let file = PartialConfig {
            host: Some("file-host".into()),
            port: Some(3000),
            feature_x: Some(false),
        };
        let env = PartialConfig {
            host: Some("env-host".into()),
            port: None,
            feature_x: Some(true),
        };
        let cli = PartialConfig {
            host: None,
            port: Some(7000),
            feature_x: None,
        };

        let merged = merge_config(base(), Some(file), Some(env), Some(cli));
        assert_eq!(
            merged,
            Config {
                host: "env-host".into(),
                port: 7000,
                feature_x: true,
            }
        );
    }

    #[test]
    fn env_false_overrides_file_true() {
        let file = PartialConfig {
            host: None,
            port: None,
            feature_x: Some(true),
        };
        let env = PartialConfig {
            host: None,
            port: None,
            feature_x: Some(false),
        };

        let merged = merge_config(base(), Some(file), Some(env), None);
        assert_eq!(merged.feature_x, false);
    }

    #[test]
    fn absent_env_field_does_not_reset_prior_values() {
        let file = PartialConfig {
            host: Some("file-host".into()),
            port: Some(9090),
            feature_x: Some(true),
        };
        let env = PartialConfig {
            host: None,
            port: None,
            feature_x: None,
        };

        let merged = merge_config(base(), Some(file), Some(env), None);
        assert_eq!(
            merged,
            Config {
                host: "file-host".into(),
                port: 9090,
                feature_x: true,
            }
        );
    }
}
