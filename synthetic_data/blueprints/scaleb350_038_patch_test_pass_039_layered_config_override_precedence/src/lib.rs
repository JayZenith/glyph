#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub tls: bool,
    pub retries: u8,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub tls: Option<bool>,
    pub retries: Option<u8>,
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
        retries: file.retries.or(env.retries).unwrap_or(defaults.retries),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Config {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            tls: false,
            retries: 3,
        }
    }

    #[test]
    fn env_overrides_file_and_defaults() {
        let file = PartialConfig {
            host: Some("file.internal".to_string()),
            port: Some(9000),
            tls: Some(false),
            retries: Some(5),
        };
        let env = PartialConfig {
            host: Some("env.internal".to_string()),
            port: Some(7000),
            tls: Some(true),
            retries: Some(1),
        };

        let merged = merge_config(&base(), &file, &env);
        assert_eq!(
            merged,
            Config {
                host: "env.internal".to_string(),
                port: 7000,
                tls: true,
                retries: 1,
            }
        );
    }

    #[test]
    fn file_fills_missing_env_values() {
        let file = PartialConfig {
            host: Some("file.internal".to_string()),
            port: Some(9000),
            tls: Some(true),
            retries: Some(6),
        };
        let env = PartialConfig {
            host: None,
            port: Some(7000),
            tls: None,
            retries: None,
        };

        let merged = merge_config(&base(), &file, &env);
        assert_eq!(merged.host, "file.internal");
        assert_eq!(merged.port, 7000);
        assert!(merged.tls);
        assert_eq!(merged.retries, 6);
    }

    #[test]
    fn defaults_fill_when_no_other_values_exist() {
        let file = PartialConfig::default();
        let env = PartialConfig::default();

        let merged = merge_config(&base(), &file, &env);
        assert_eq!(merged, base());
    }
}
