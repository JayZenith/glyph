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
    defaults: &Config,
    file: Option<&PartialConfig>,
    env: Option<&PartialConfig>,
) -> Config {
    let mut merged = defaults.clone();

    if let Some(file) = file {
        if let Some(host) = &file.host {
            merged.host = host.clone();
        }
        if let Some(port) = file.port {
            merged.port = port;
        }
        if let Some(use_tls) = file.use_tls {
            merged.use_tls = use_tls;
        }
    }

    if let Some(env) = env {
        if let Some(host) = &env.host {
            merged.host = host.clone();
        }
        if let Some(port) = env.port {
            merged.port = port;
        }
        if let Some(use_tls) = env.use_tls {
            merged.use_tls = use_tls;
        }
    }

    merged
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
    fn env_fills_missing_file_fields() {
        let file = PartialConfig {
            host: Some("file-host".to_string()),
            port: None,
            use_tls: None,
        };
        let env = PartialConfig {
            host: None,
            port: Some(9000),
            use_tls: Some(true),
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&env));
        assert_eq!(
            merged,
            Config {
                host: "file-host".to_string(),
                port: 9000,
                use_tls: true,
            }
        );
    }

    #[test]
    fn file_values_take_precedence_over_env() {
        let file = PartialConfig {
            host: Some("file-host".to_string()),
            port: Some(7000),
            use_tls: Some(false),
        };
        let env = PartialConfig {
            host: Some("env-host".to_string()),
            port: Some(9000),
            use_tls: Some(true),
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&env));
        assert_eq!(
            merged,
            Config {
                host: "file-host".to_string(),
                port: 7000,
                use_tls: false,
            }
        );
    }

    #[test]
    fn env_overrides_defaults_without_file() {
        let env = PartialConfig {
            host: Some("env-host".to_string()),
            port: Some(9090),
            use_tls: Some(true),
        };

        let merged = merge_config(&defaults(), None, Some(&env));
        assert_eq!(
            merged,
            Config {
                host: "env-host".to_string(),
                port: 9090,
                use_tls: true,
            }
        );
    }
}
