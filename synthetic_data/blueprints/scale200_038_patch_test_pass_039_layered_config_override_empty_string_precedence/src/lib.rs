#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub prefix: String,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub prefix: Option<String>,
}

pub fn merge_config(
    defaults: &Config,
    file_cfg: Option<&PartialConfig>,
    env_cfg: Option<&PartialConfig>,
) -> Config {
    let mut merged = defaults.clone();

    if let Some(file) = file_cfg {
        if let Some(host) = &file.host {
            merged.host = host.clone();
        }
        if let Some(port) = file.port {
            merged.port = port;
        }
        if let Some(prefix) = &file.prefix {
            merged.prefix = prefix.clone();
        }
    }

    if let Some(env) = env_cfg {
        if let Some(host) = &env.host {
            merged.host = host.clone();
        }
        if let Some(port) = env.port {
            merged.port = port;
        }
        if let Some(prefix) = &env.prefix {
            if !prefix.is_empty() {
                merged.prefix = prefix.clone();
            }
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
            prefix: "/api".to_string(),
        }
    }

    #[test]
    fn file_overrides_defaults_when_env_absent() {
        let file = PartialConfig {
            host: Some("file-host".to_string()),
            port: Some(9000),
            prefix: Some("/v1".to_string()),
        };

        let merged = merge_config(&defaults(), Some(&file), None);

        assert_eq!(
            merged,
            Config {
                host: "file-host".to_string(),
                port: 9000,
                prefix: "/v1".to_string(),
            }
        );
    }

    #[test]
    fn env_overrides_file_values() {
        let file = PartialConfig {
            host: Some("file-host".to_string()),
            port: Some(9000),
            prefix: Some("/v1".to_string()),
        };
        let env = PartialConfig {
            host: Some("env-host".to_string()),
            port: Some(7000),
            prefix: Some("/env".to_string()),
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&env));

        assert_eq!(
            merged,
            Config {
                host: "env-host".to_string(),
                port: 7000,
                prefix: "/env".to_string(),
            }
        );
    }

    #[test]
    fn missing_env_values_do_not_erase_lower_precedence_values() {
        let file = PartialConfig {
            host: Some("file-host".to_string()),
            port: Some(9000),
            prefix: Some("/v1".to_string()),
        };
        let env = PartialConfig {
            host: None,
            port: None,
            prefix: None,
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&env));

        assert_eq!(
            merged,
            Config {
                host: "file-host".to_string(),
                port: 9000,
                prefix: "/v1".to_string(),
            }
        );
    }

    #[test]
    fn explicit_empty_env_prefix_still_overrides_file_and_default() {
        let file = PartialConfig {
            host: None,
            port: None,
            prefix: Some("/from-file".to_string()),
        };
        let env = PartialConfig {
            host: None,
            port: None,
            prefix: Some(String::new()),
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&env));

        assert_eq!(merged.prefix, "");
    }
}
