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

pub fn merge_config(defaults: Config, file: PartialConfig, env: PartialConfig) -> Config {
    Config {
        host: file.host.or(env.host).unwrap_or(defaults.host),
        port: file.port.or(env.port).unwrap_or(defaults.port),
        use_tls: file.use_tls.unwrap_or(defaults.use_tls),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            use_tls: false,
        }
    }

    #[test]
    fn file_values_override_defaults() {
        let cfg = merge_config(
            defaults(),
            PartialConfig {
                host: Some("file.internal".into()),
                port: Some(9000),
                use_tls: Some(true),
            },
            PartialConfig::default(),
        );

        assert_eq!(
            cfg,
            Config {
                host: "file.internal".into(),
                port: 9000,
                use_tls: true,
            }
        );
    }

    #[test]
    fn env_overrides_file_and_defaults() {
        let cfg = merge_config(
            defaults(),
            PartialConfig {
                host: Some("file.internal".into()),
                port: Some(9000),
                use_tls: Some(false),
            },
            PartialConfig {
                host: Some("env.internal".into()),
                port: Some(9443),
                use_tls: Some(true),
            },
        );

        assert_eq!(
            cfg,
            Config {
                host: "env.internal".into(),
                port: 9443,
                use_tls: true,
            }
        );
    }

    #[test]
    fn env_false_must_still_override_file_true() {
        let cfg = merge_config(
            defaults(),
            PartialConfig {
                host: None,
                port: None,
                use_tls: Some(true),
            },
            PartialConfig {
                host: None,
                port: None,
                use_tls: Some(false),
            },
        );

        assert!(!cfg.use_tls);
    }

    #[test]
    fn missing_values_fall_back_by_layer() {
        let cfg = merge_config(
            defaults(),
            PartialConfig {
                host: None,
                port: Some(7000),
                use_tls: None,
            },
            PartialConfig {
                host: Some("env-only".into()),
                port: None,
                use_tls: None,
            },
        );

        assert_eq!(
            cfg,
            Config {
                host: "env-only".into(),
                port: 7000,
                use_tls: false,
            }
        );
    }
}
