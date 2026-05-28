#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub debug: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            debug: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub debug: Option<bool>,
}

pub fn merge_config(defaults: AppConfig, env: PartialConfig, overrides: PartialConfig) -> AppConfig {
    AppConfig {
        host: env.host.or(overrides.host).unwrap_or(defaults.host),
        port: env.port.or(overrides.port).unwrap_or(defaults.port),
        debug: env.debug.or(overrides.debug).unwrap_or(defaults.debug),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_defaults_when_nothing_else_is_set() {
        let merged = merge_config(AppConfig::default(), PartialConfig::default(), PartialConfig::default());
        assert_eq!(
            merged,
            AppConfig {
                host: "127.0.0.1".into(),
                port: 8080,
                debug: false,
            }
        );
    }

    #[test]
    fn uses_env_when_override_missing() {
        let merged = merge_config(
            AppConfig::default(),
            PartialConfig {
                host: Some("env.local".into()),
                port: Some(9000),
                debug: None,
            },
            PartialConfig::default(),
        );

        assert_eq!(merged.host, "env.local");
        assert_eq!(merged.port, 9000);
        assert!(!merged.debug);
    }

    #[test]
    fn explicit_override_has_highest_precedence() {
        let merged = merge_config(
            AppConfig::default(),
            PartialConfig {
                host: Some("env.local".into()),
                port: Some(9000),
                debug: Some(false),
            },
            PartialConfig {
                host: Some("cli.local".into()),
                port: Some(7000),
                debug: Some(true),
            },
        );

        assert_eq!(
            merged,
            AppConfig {
                host: "cli.local".into(),
                port: 7000,
                debug: true,
            }
        );
    }
}
