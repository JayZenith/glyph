#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub color: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub color: Option<bool>,
}

pub fn merge_config(defaults: PartialConfig, env: PartialConfig, cli: PartialConfig) -> Config {
    let host = defaults
        .host
        .or(env.host)
        .or(cli.host)
        .unwrap_or_else(|| "127.0.0.1".to_string());

    let port = defaults.port.or(env.port).or(cli.port).unwrap_or(8080);

    let color = defaults.color.unwrap_or(false)
        || env.color.unwrap_or(false)
        || cli.color.unwrap_or(false);

    Config { host, port, color }
}

#[cfg(test)]
mod tests {
    use super::{merge_config, Config, PartialConfig};

    #[test]
    fn cli_overrides_env_and_defaults() {
        let defaults = PartialConfig {
            host: Some("def.local".into()),
            port: Some(8000),
            color: Some(false),
        };
        let env = PartialConfig {
            host: Some("env.local".into()),
            port: Some(9000),
            color: Some(true),
        };
        let cli = PartialConfig {
            host: Some("cli.local".into()),
            port: Some(7000),
            color: Some(false),
        };

        assert_eq!(
            merge_config(defaults, env, cli),
            Config {
                host: "cli.local".into(),
                port: 7000,
                color: false,
            }
        );
    }

    #[test]
    fn env_used_when_cli_missing() {
        let defaults = PartialConfig {
            host: Some("def.local".into()),
            port: Some(8000),
            color: Some(false),
        };
        let env = PartialConfig {
            host: None,
            port: Some(9001),
            color: Some(true),
        };
        let cli = PartialConfig::default();

        assert_eq!(
            merge_config(defaults, env, cli),
            Config {
                host: "def.local".into(),
                port: 9001,
                color: true,
            }
        );
    }

    #[test]
    fn explicit_false_from_higher_precedence_wins() {
        let defaults = PartialConfig {
            host: None,
            port: None,
            color: Some(true),
        };
        let env = PartialConfig {
            host: None,
            port: None,
            color: Some(true),
        };
        let cli = PartialConfig {
            host: None,
            port: None,
            color: Some(false),
        };

        assert_eq!(
            merge_config(defaults, env, cli),
            Config {
                host: "127.0.0.1".into(),
                port: 8080,
                color: false,
            }
        );
    }
}
