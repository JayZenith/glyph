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

pub fn merge_config(file: PartialConfig, env: PartialConfig) -> Config {
    let defaults = Config::defaults();
    Config {
        host: file.host.or(env.host).unwrap_or(defaults.host),
        port: file.port.or(env.port).unwrap_or(defaults.port),
        use_tls: file.use_tls.or(env.use_tls).unwrap_or(defaults.use_tls),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_overrides_file_values() {
        let file = PartialConfig {
            host: Some("file.internal".into()),
            port: Some(7000),
            use_tls: Some(false),
        };
        let env = PartialConfig {
            host: Some("env.prod".into()),
            port: Some(443),
            use_tls: Some(true),
        };

        let merged = merge_config(file, env);
        assert_eq!(
            merged,
            Config {
                host: "env.prod".into(),
                port: 443,
                use_tls: true,
            }
        );
    }

    #[test]
    fn file_fills_missing_env_fields_and_defaults_fill_rest() {
        let file = PartialConfig {
            host: Some("cfg.service".into()),
            port: None,
            use_tls: Some(true),
        };
        let env = PartialConfig {
            host: None,
            port: Some(9001),
            use_tls: None,
        };

        let merged = merge_config(file, env);
        assert_eq!(merged.host, "cfg.service");
        assert_eq!(merged.port, 9001);
        assert!(merged.use_tls);
    }

    #[test]
    fn explicit_false_from_env_must_override_true_in_file() {
        let file = PartialConfig {
            host: None,
            port: None,
            use_tls: Some(true),
        };
        let env = PartialConfig {
            host: None,
            port: None,
            use_tls: Some(false),
        };

        let merged = merge_config(file, env);
        assert!(!merged.use_tls);
    }
}
