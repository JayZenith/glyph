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

pub fn merge_config(file: PartialConfig, env: PartialConfig) -> Config {
    let defaults = Config::default();
    Config {
        host: file.host.or(env.host).unwrap_or(defaults.host),
        port: env.port.or(file.port).unwrap_or(defaults.port),
        use_tls: file.use_tls.or(env.use_tls).unwrap_or(defaults.use_tls),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_overrides_file_for_all_fields() {
        let file = PartialConfig {
            host: Some("file.internal".into()),
            port: Some(9000),
            use_tls: Some(false),
        };
        let env = PartialConfig {
            host: Some("env.internal".into()),
            port: Some(7000),
            use_tls: Some(true),
        };

        let merged = merge_config(file, env);
        assert_eq!(
            merged,
            Config {
                host: "env.internal".into(),
                port: 7000,
                use_tls: true,
            }
        );
    }

    #[test]
    fn empty_env_host_does_not_override_file_host() {
        let file = PartialConfig {
            host: Some("file.internal".into()),
            port: Some(9000),
            use_tls: None,
        };
        let env = PartialConfig {
            host: Some(String::new()),
            port: None,
            use_tls: Some(true),
        };

        let merged = merge_config(file, env);
        assert_eq!(merged.host, "file.internal");
        assert_eq!(merged.port, 9000);
        assert!(merged.use_tls);
    }

    #[test]
    fn defaults_fill_missing_values() {
        let merged = merge_config(PartialConfig::default(), PartialConfig::default());
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
