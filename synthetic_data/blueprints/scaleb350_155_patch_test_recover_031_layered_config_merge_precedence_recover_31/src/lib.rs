#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub use_tls: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub use_tls: Option<bool>,
}

impl Config {
    pub fn defaults() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
            use_tls: false,
        }
    }
}

pub fn merge_config(
    defaults: Config,
    file_cfg: PartialConfig,
    env_cfg: PartialConfig,
    cli_cfg: PartialConfig,
) -> Config {
    Config {
        host: cli_cfg
            .host
            .or(env_cfg.host)
            .or(file_cfg.host)
            .unwrap_or(defaults.host),
        port: file_cfg
            .port
            .or(env_cfg.port)
            .or(cli_cfg.port)
            .unwrap_or(defaults.port),
        use_tls: file_cfg
            .use_tls
            .or(cli_cfg.use_tls)
            .or(env_cfg.use_tls)
            .unwrap_or(defaults.use_tls),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn partial(host: Option<&str>, port: Option<u16>, use_tls: Option<bool>) -> PartialConfig {
        PartialConfig {
            host: host.map(str::to_string),
            port,
            use_tls,
        }
    }

    #[test]
    fn uses_defaults_when_nothing_else_is_set() {
        let merged = merge_config(
            Config::defaults(),
            PartialConfig::default(),
            PartialConfig::default(),
            PartialConfig::default(),
        );

        assert_eq!(
            merged,
            Config {
                host: "localhost".to_string(),
                port: 8080,
                use_tls: false,
            }
        );
    }

    #[test]
    fn precedence_is_cli_then_env_then_file_then_defaults() {
        let merged = merge_config(
            Config::defaults(),
            partial(Some("file.internal"), Some(7000), Some(false)),
            partial(Some("env.internal"), Some(9000), Some(true)),
            partial(Some("cli.internal"), Some(10000), Some(false)),
        );

        assert_eq!(merged.host, "cli.internal");
        assert_eq!(merged.port, 10000);
        assert!(!merged.use_tls);
    }

    #[test]
    fn lower_layers_fill_only_missing_values() {
        let merged = merge_config(
            Config::defaults(),
            partial(Some("file.internal"), Some(7000), None),
            partial(None, Some(9000), Some(true)),
            partial(None, None, None),
        );

        assert_eq!(
            merged,
            Config {
                host: "file.internal".to_string(),
                port: 9000,
                use_tls: true,
            }
        );
    }

    #[test]
    fn explicit_false_from_higher_layer_must_override_true_below() {
        let merged = merge_config(
            Config::defaults(),
            partial(None, None, Some(true)),
            partial(None, None, Some(true)),
            partial(None, None, Some(false)),
        );

        assert!(!merged.use_tls);
    }
}
