#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub verbose: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub verbose: Option<bool>,
}

pub fn merge_config(defaults: Config, file: PartialConfig, cli: PartialConfig) -> Config {
    Config {
        host: defaults.host,
        port: cli.port.or(file.port).unwrap_or(defaults.port),
        verbose: file.verbose.or(cli.verbose).unwrap_or(defaults.verbose),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "127.0.0.1".to_string(),
            port: 8080,
            verbose: false,
        }
    }

    #[test]
    fn cli_should_override_file_for_all_fields() {
        let merged = merge_config(
            defaults(),
            PartialConfig {
                host: Some("file.local".to_string()),
                port: Some(3000),
                verbose: Some(false),
            },
            PartialConfig {
                host: Some("cli.local".to_string()),
                port: Some(9000),
                verbose: Some(true),
            },
        );

        assert_eq!(
            merged,
            Config {
                host: "cli.local".to_string(),
                port: 9000,
                verbose: true,
            }
        );
    }

    #[test]
    fn file_values_fill_missing_cli_values() {
        let merged = merge_config(
            defaults(),
            PartialConfig {
                host: Some("file.local".to_string()),
                port: Some(3000),
                verbose: Some(true),
            },
            PartialConfig {
                host: None,
                port: Some(9000),
                verbose: None,
            },
        );

        assert_eq!(
            merged,
            Config {
                host: "file.local".to_string(),
                port: 9000,
                verbose: true,
            }
        );
    }

    #[test]
    fn defaults_are_used_only_when_both_layers_are_missing() {
        let merged = merge_config(
            defaults(),
            PartialConfig::default(),
            PartialConfig {
                host: None,
                port: None,
                verbose: Some(true),
            },
        );

        assert_eq!(
            merged,
            Config {
                host: "127.0.0.1".to_string(),
                port: 8080,
                verbose: true,
            }
        );
    }
}
