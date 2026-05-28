#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
            debug: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub debug: Option<bool>,
}

pub fn merge_config(defaults: Config, env: PartialConfig, cli: PartialConfig) -> Config {
    Config {
        host: env.host.or(cli.host).unwrap_or(defaults.host),
        port: env.port.or(cli.port).unwrap_or(defaults.port),
        debug: env.debug.or(cli.debug).unwrap_or(defaults.debug),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_overrides_env_and_defaults() {
        let defaults = Config::default();
        let env = PartialConfig {
            host: Some("env.example.com".to_string()),
            port: Some(9000),
            debug: Some(false),
        };
        let cli = PartialConfig {
            host: Some("cli.example.com".to_string()),
            port: Some(7000),
            debug: Some(true),
        };

        let merged = merge_config(defaults, env, cli);
        assert_eq!(
            merged,
            Config {
                host: "cli.example.com".to_string(),
                port: 7000,
                debug: true,
            }
        );
    }

    #[test]
    fn env_overrides_defaults_when_cli_missing() {
        let defaults = Config::default();
        let env = PartialConfig {
            host: Some("env.example.com".to_string()),
            port: Some(9000),
            debug: Some(true),
        };
        let cli = PartialConfig::default();

        let merged = merge_config(defaults, env, cli);
        assert_eq!(
            merged,
            Config {
                host: "env.example.com".to_string(),
                port: 9000,
                debug: true,
            }
        );
    }

    #[test]
    fn defaults_fill_missing_values() {
        let defaults = Config::default();
        let env = PartialConfig {
            host: None,
            port: Some(9000),
            debug: None,
        };
        let cli = PartialConfig {
            host: None,
            port: None,
            debug: None,
        };

        let merged = merge_config(defaults, env, cli);
        assert_eq!(
            merged,
            Config {
                host: "localhost".to_string(),
                port: 9000,
                debug: false,
            }
        );
    }
}
