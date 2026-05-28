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

pub fn merge_config(
    defaults: PartialConfig,
    file: PartialConfig,
    env: PartialConfig,
) -> Config {
    let mut merged = defaults;
    merged = file;
    merged = env;

    Config {
        host: merged.host.unwrap_or_else(|| "127.0.0.1".to_string()),
        port: merged.port.unwrap_or(8080),
        use_tls: merged.use_tls.unwrap_or(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_overrides_file_and_defaults_without_clearing_other_values() {
        let defaults = PartialConfig {
            host: Some("localhost".into()),
            port: Some(80),
            use_tls: Some(false),
        };
        let file = PartialConfig {
            host: Some("file.internal".into()),
            port: Some(443),
            use_tls: None,
        };
        let env = PartialConfig {
            host: None,
            port: Some(8443),
            use_tls: Some(true),
        };

        let cfg = merge_config(defaults, file, env);

        assert_eq!(
            cfg,
            Config {
                host: "file.internal".into(),
                port: 8443,
                use_tls: true,
            }
        );
    }

    #[test]
    fn falls_back_to_defaults_when_upper_layers_are_unset() {
        let defaults = PartialConfig {
            host: Some("default.internal".into()),
            port: Some(3000),
            use_tls: Some(false),
        };
        let file = PartialConfig::default();
        let env = PartialConfig {
            host: None,
            port: None,
            use_tls: Some(true),
        };

        let cfg = merge_config(defaults, file, env);

        assert_eq!(cfg.host, "default.internal");
        assert_eq!(cfg.port, 3000);
        assert!(cfg.use_tls);
    }

    #[test]
    fn uses_builtin_fallbacks_when_no_layer_sets_a_value() {
        let cfg = merge_config(
            PartialConfig::default(),
            PartialConfig::default(),
            PartialConfig::default(),
        );

        assert_eq!(
            cfg,
            Config {
                host: "127.0.0.1".into(),
                port: 8080,
                use_tls: false,
            }
        );
    }
}
