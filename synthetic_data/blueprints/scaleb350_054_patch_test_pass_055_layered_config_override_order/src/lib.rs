#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub use_tls: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
            use_tls: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub use_tls: Option<bool>,
}

pub fn merge_config(defaults: Config, env: PartialConfig, cli: PartialConfig) -> Config {
    Config {
        host: defaults
            .host,
        port: cli
            .port
            .or(env.port)
            .unwrap_or(defaults.port),
        use_tls: env
            .use_tls
            .or(cli.use_tls)
            .unwrap_or(defaults.use_tls),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_values_override_env_and_defaults() {
        let defaults = Config::default();
        let env = PartialConfig {
            host: Some("env.internal".into()),
            port: Some(9000),
            use_tls: Some(false),
        };
        let cli = PartialConfig {
            host: Some("cli.example.com".into()),
            port: Some(7000),
            use_tls: Some(true),
        };

        let merged = merge_config(defaults, env, cli);
        assert_eq!(
            merged,
            Config {
                host: "cli.example.com".into(),
                port: 7000,
                use_tls: true,
            }
        );
    }

    #[test]
    fn env_values_fill_missing_cli_values() {
        let defaults = Config::default();
        let env = PartialConfig {
            host: Some("env.internal".into()),
            port: Some(9000),
            use_tls: Some(true),
        };
        let cli = PartialConfig {
            host: None,
            port: Some(7000),
            use_tls: None,
        };

        let merged = merge_config(defaults, env, cli);
        assert_eq!(merged.host, "env.internal");
        assert_eq!(merged.port, 7000);
        assert!(merged.use_tls);
    }

    #[test]
    fn defaults_are_used_when_no_override_exists() {
        let merged = merge_config(Config::default(), PartialConfig::default(), PartialConfig::default());
        assert_eq!(
            merged,
            Config {
                host: "localhost".into(),
                port: 8080,
                use_tls: false,
            }
        );
    }
}
