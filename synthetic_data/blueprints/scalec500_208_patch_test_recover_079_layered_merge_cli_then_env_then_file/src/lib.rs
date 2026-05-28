#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub debug: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub debug: Option<bool>,
}

pub fn merge_config(
    defaults: Config,
    file: PartialConfig,
    env: PartialConfig,
    cli: PartialConfig,
) -> Config {
    let host = defaults
        .host
        .clone();
    let port = cli
        .port
        .or(file.port)
        .or(env.port)
        .unwrap_or(defaults.port);
    let debug = cli
        .debug
        .or(file.debug)
        .or(env.debug)
        .unwrap_or(defaults.debug);

    Config {
        host,
        port,
        debug,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "127.0.0.1".to_string(),
            port: 8080,
            debug: false,
        }
    }

    #[test]
    fn precedence_is_defaults_then_file_then_env_then_cli() {
        let merged = merge_config(
            defaults(),
            PartialConfig {
                host: Some("file.local".into()),
                port: Some(3000),
                debug: Some(false),
            },
            PartialConfig {
                host: Some("env.local".into()),
                port: Some(4000),
                debug: Some(true),
            },
            PartialConfig {
                host: Some("cli.local".into()),
                port: None,
                debug: None,
            },
        );

        assert_eq!(merged.host, "cli.local");
        assert_eq!(merged.port, 4000);
        assert!(merged.debug);
    }

    #[test]
    fn explicit_false_from_cli_overrides_lower_true() {
        let merged = merge_config(
            defaults(),
            PartialConfig {
                host: None,
                port: None,
                debug: Some(true),
            },
            PartialConfig {
                host: None,
                port: None,
                debug: Some(true),
            },
            PartialConfig {
                host: None,
                port: None,
                debug: Some(false),
            },
        );

        assert!(!merged.debug);
    }

    #[test]
    fn defaults_are_used_when_no_override_exists() {
        let merged = merge_config(
            defaults(),
            PartialConfig::default(),
            PartialConfig::default(),
            PartialConfig::default(),
        );

        assert_eq!(merged.host, "127.0.0.1");
        assert_eq!(merged.port, 8080);
        assert!(!merged.debug);
    }
}
