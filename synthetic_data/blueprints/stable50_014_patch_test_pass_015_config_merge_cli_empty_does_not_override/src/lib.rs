#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub log_level: String,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub log_level: Option<String>,
}

pub fn merge_config(defaults: Config, file: PartialConfig, cli: PartialConfig) -> Config {
    Config {
        host: cli
            .host
            .or(file.host)
            .unwrap_or(defaults.host),
        port: cli
            .port
            .or(file.port)
            .unwrap_or(defaults.port),
        log_level: cli
            .log_level
            .or(file.log_level)
            .unwrap_or(defaults.log_level),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "127.0.0.1".into(),
            port: 8080,
            log_level: "info".into(),
        }
    }

    #[test]
    fn cli_overrides_file_and_defaults() {
        let file = PartialConfig {
            host: Some("file-host".into()),
            port: Some(9000),
            log_level: Some("warn".into()),
        };
        let cli = PartialConfig {
            host: Some("cli-host".into()),
            port: Some(7000),
            log_level: Some("debug".into()),
        };

        let merged = merge_config(defaults(), file, cli);
        assert_eq!(
            merged,
            Config {
                host: "cli-host".into(),
                port: 7000,
                log_level: "debug".into(),
            }
        );
    }

    #[test]
    fn file_fills_missing_cli_values() {
        let file = PartialConfig {
            host: Some("file-host".into()),
            port: Some(9000),
            log_level: None,
        };
        let cli = PartialConfig {
            host: None,
            port: None,
            log_level: Some("error".into()),
        };

        let merged = merge_config(defaults(), file, cli);
        assert_eq!(
            merged,
            Config {
                host: "file-host".into(),
                port: 9000,
                log_level: "error".into(),
            }
        );
    }

    #[test]
    fn defaults_fill_missing_file_and_cli_values() {
        let merged = merge_config(defaults(), PartialConfig::default(), PartialConfig::default());
        assert_eq!(
            merged,
            Config {
                host: "127.0.0.1".into(),
                port: 8080,
                log_level: "info".into(),
            }
        );
    }

    #[test]
    fn empty_cli_host_and_log_level_do_not_override_file_values() {
        let file = PartialConfig {
            host: Some("file-host".into()),
            port: Some(9000),
            log_level: Some("warn".into()),
        };
        let cli = PartialConfig {
            host: Some(String::new()),
            port: None,
            log_level: Some(String::new()),
        };

        let merged = merge_config(defaults(), file, cli);
        assert_eq!(merged.host, "file-host");
        assert_eq!(merged.port, 9000);
        assert_eq!(merged.log_level, "warn");
    }

    #[test]
    fn empty_cli_host_and_log_level_fall_back_to_defaults_when_file_missing() {
        let cli = PartialConfig {
            host: Some(String::new()),
            port: Some(3000),
            log_level: Some(String::new()),
        };

        let merged = merge_config(defaults(), PartialConfig::default(), cli);
        assert_eq!(merged.host, "127.0.0.1");
        assert_eq!(merged.port, 3000);
        assert_eq!(merged.log_level, "info");
    }
}
