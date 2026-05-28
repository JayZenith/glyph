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
        if let Some(tls) = file.tls {
            merged.tls = tls;
        }
    }

    if let Some(env) = env_cfg {
        if let Some(host) = &env.host {
            merged.host = host.clone();
        }
        if let Some(port) = env.port {
            merged.port = port;
        }
        if let Some(tls) = env.tls {
            merged.tls = tls;
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
            tls: false,
        }
    }

    #[test]
    fn uses_defaults_when_no_overrides_exist() {
        let merged = merge_config(&defaults(), None, None);
        assert_eq!(merged, defaults());
    }

    #[test]
    fn file_values_override_defaults() {
        let file = PartialConfig {
            host: Some("file.internal".to_string()),
            port: Some(9000),
            tls: None,
        };

        let merged = merge_config(&defaults(), Some(&file), None);

        assert_eq!(
            merged,
            Config {
                host: "file.internal".to_string(),
                port: 9000,
                tls: false,
            }
        );
    }

    #[test]
    fn env_values_override_file_and_defaults() {
        let file = PartialConfig {
            host: Some("file.internal".to_string()),
            port: Some(9000),
            tls: Some(false),
        };
        let env = PartialConfig {
            host: None,
            port: Some(7000),
            tls: Some(true),
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&env));

        assert_eq!(
            merged,
            Config {
                host: "file.internal".to_string(),
                port: 7000,
                tls: true,
            }
        );
    }

    #[test]
    fn absent_env_fields_do_not_erase_file_values() {
        let file = PartialConfig {
            host: Some("cfg.service".to_string()),
            port: None,
            tls: Some(true),
        };
        let env = PartialConfig {
            host: None,
            port: Some(8443),
            tls: None,
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&env));

        assert_eq!(
            merged,
            Config {
                host: "cfg.service".to_string(),
                port: 8443,
                tls: true,
            }
        );
    }
}
