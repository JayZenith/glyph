#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub host: String,
    pub port: u16,
    pub tls: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialSettings {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub tls: Option<bool>,
}

pub fn merge_settings(
    defaults: &Settings,
    file: Option<&PartialSettings>,
    env: Option<&PartialSettings>,
) -> Settings {
    let mut merged = defaults.clone();

    if let Some(file) = file {
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

    if let Some(env) = env {
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

    if merged.host.is_empty() {
        merged.host = defaults.host.clone();
    }
    if merged.port == 0 {
        merged.port = defaults.port;
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Settings {
        Settings {
            host: "localhost".to_string(),
            port: 8080,
            tls: false,
        }
    }

    #[test]
    fn file_overrides_defaults() {
        let file = PartialSettings {
            host: Some("file-host".to_string()),
            port: Some(9000),
            tls: Some(true),
        };

        let merged = merge_settings(&defaults(), Some(&file), None);
        assert_eq!(
            merged,
            Settings {
                host: "file-host".to_string(),
                port: 9000,
                tls: true,
            }
        );
    }

    #[test]
    fn env_overrides_file() {
        let file = PartialSettings {
            host: Some("file-host".to_string()),
            port: Some(9000),
            tls: Some(false),
        };
        let env = PartialSettings {
            host: Some("env-host".to_string()),
            port: None,
            tls: Some(true),
        };

        let merged = merge_settings(&defaults(), Some(&file), Some(&env));
        assert_eq!(
            merged,
            Settings {
                host: "env-host".to_string(),
                port: 9000,
                tls: true,
            }
        );
    }

    #[test]
    fn empty_env_host_does_not_clear_lower_precedence_value() {
        let file = PartialSettings {
            host: Some("file-host".to_string()),
            port: Some(7000),
            tls: None,
        };
        let env = PartialSettings {
            host: Some(String::new()),
            port: None,
            tls: None,
        };

        let merged = merge_settings(&defaults(), Some(&file), Some(&env));
        assert_eq!(merged.host, "file-host");
        assert_eq!(merged.port, 7000);
    }

    #[test]
    fn zero_env_port_does_not_fall_back_to_default_when_file_has_value() {
        let file = PartialSettings {
            host: None,
            port: Some(7000),
            tls: None,
        };
        let env = PartialSettings {
            host: None,
            port: Some(0),
            tls: None,
        };

        let merged = merge_settings(&defaults(), Some(&file), Some(&env));
        assert_eq!(merged.port, 7000);
    }
}
