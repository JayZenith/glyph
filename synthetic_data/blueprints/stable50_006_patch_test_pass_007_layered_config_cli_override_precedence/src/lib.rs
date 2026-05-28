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
            host: "127.0.0.1".to_string(),
            port: 8080,
            use_tls: false,
        }
    }
}

pub fn merge_config(
    file: PartialConfig,
    env: PartialConfig,
    cli: PartialConfig,
) -> Config {
    let mut cfg = Config::defaults();

    if let Some(host) = file.host {
        cfg.host = host;
    }
    if let Some(port) = file.port {
        cfg.port = port;
    }
    if let Some(use_tls) = file.use_tls {
        cfg.use_tls = use_tls;
    }

    if let Some(host) = env.host {
        cfg.host = host;
    }
    if let Some(port) = env.port {
        cfg.port = port;
    }
    if let Some(use_tls) = env.use_tls {
        cfg.use_tls = use_tls;
    }

    cfg.host = cli.host.unwrap_or_default();
    cfg.port = cli.port.unwrap_or_default();
    cfg.use_tls = cli.use_tls.unwrap_or(false);

    cfg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_overrides_only_specified_fields() {
        let cfg = merge_config(
            PartialConfig {
                host: Some("file.local".into()),
                port: Some(7000),
                use_tls: Some(false),
            },
            PartialConfig {
                host: Some("env.local".into()),
                port: None,
                use_tls: Some(true),
            },
            PartialConfig {
                host: None,
                port: Some(9000),
                use_tls: None,
            },
        );

        assert_eq!(
            cfg,
            Config {
                host: "env.local".into(),
                port: 9000,
                use_tls: true,
            }
        );
    }

    #[test]
    fn missing_cli_keeps_lower_precedence_values() {
        let cfg = merge_config(
            PartialConfig {
                host: Some("file.local".into()),
                port: Some(7000),
                use_tls: None,
            },
            PartialConfig::default(),
            PartialConfig::default(),
        );

        assert_eq!(
            cfg,
            Config {
                host: "file.local".into(),
                port: 7000,
                use_tls: false,
            }
        );
    }

    #[test]
    fn explicit_false_from_cli_still_overrides() {
        let cfg = merge_config(
            PartialConfig {
                host: None,
                port: None,
                use_tls: Some(true),
            },
            PartialConfig::default(),
            PartialConfig {
                host: None,
                port: None,
                use_tls: Some(false),
            },
        );

        assert!(!cfg.use_tls);
    }
}
