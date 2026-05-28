#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub tls: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub tls: Option<bool>,
}

pub fn merge_config(defaults: &Config, file: &PartialConfig, env: &PartialConfig) -> Config {
    Config {
        host: env
            .host
            .clone()
            .or_else(|| file.host.clone())
            .unwrap_or_else(|| defaults.host.clone()),
        port: env.port.or(file.port).unwrap_or(defaults.port),
        tls: env.tls.or(file.tls).unwrap_or(defaults.tls),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Config {
        Config {
            host: "127.0.0.1".to_string(),
            port: 8080,
            tls: false,
        }
    }

    #[test]
    fn file_overrides_defaults_when_env_missing() {
        let file = PartialConfig {
            host: Some("file.local".to_string()),
            port: Some(9000),
            tls: Some(true),
        };
        let env = PartialConfig::default();

        let merged = merge_config(&base(), &file, &env);
        assert_eq!(
            merged,
            Config {
                host: "file.local".to_string(),
                port: 9000,
                tls: true,
            }
        );
    }

    #[test]
    fn env_only_fills_missing_file_values() {
        let file = PartialConfig {
            host: Some("file.local".to_string()),
            port: None,
            tls: Some(false),
        };
        let env = PartialConfig {
            host: Some("env.local".to_string()),
            port: Some(7000),
            tls: Some(true),
        };

        let merged = merge_config(&base(), &file, &env);
        assert_eq!(merged.host, "file.local");
        assert_eq!(merged.port, 7000);
        assert!(!merged.tls);
    }

    #[test]
    fn defaults_fill_remaining_gaps() {
        let file = PartialConfig {
            host: None,
            port: None,
            tls: Some(true),
        };
        let env = PartialConfig {
            host: None,
            port: Some(5000),
            tls: None,
        };

        let merged = merge_config(&base(), &file, &env);
        assert_eq!(merged.host, "127.0.0.1");
        assert_eq!(merged.port, 5000);
        assert!(merged.tls);
    }
}
