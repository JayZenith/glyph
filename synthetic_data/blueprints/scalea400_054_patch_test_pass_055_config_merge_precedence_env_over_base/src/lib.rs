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
    pub fn merge(base: Config, env: PartialConfig, overrides: PartialConfig) -> Config {
        Config {
            host: overrides
                .host
                .or(base.host.into())
                .or(env.host)
                .unwrap(),
            port: overrides.port.or(base.port.into()).or(env.port).unwrap(),
            use_tls: overrides
                .use_tls
                .or(base.use_tls.into())
                .or(env.use_tls)
                .unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Config {
        Config {
            host: "base.local".to_string(),
            port: 8080,
            use_tls: false,
        }
    }

    #[test]
    fn env_overrides_base_when_no_explicit_override() {
        let env = PartialConfig {
            host: Some("env.local".to_string()),
            port: Some(9090),
            use_tls: Some(true),
        };
        let overrides = PartialConfig::default();

        let merged = Config::merge(base(), env, overrides);

        assert_eq!(
            merged,
            Config {
                host: "env.local".to_string(),
                port: 9090,
                use_tls: true,
            }
        );
    }

    #[test]
    fn explicit_overrides_win_but_only_for_fields_present() {
        let env = PartialConfig {
            host: Some("env.local".to_string()),
            port: Some(9090),
            use_tls: Some(true),
        };
        let overrides = PartialConfig {
            host: None,
            port: Some(7000),
            use_tls: None,
        };

        let merged = Config::merge(base(), env, overrides);

        assert_eq!(
            merged,
            Config {
                host: "env.local".to_string(),
                port: 7000,
                use_tls: true,
            }
        );
    }

    #[test]
    fn base_is_used_when_neither_env_nor_overrides_set_field() {
        let env = PartialConfig {
            host: None,
            port: Some(9090),
            use_tls: None,
        };
        let overrides = PartialConfig {
            host: None,
            port: None,
            use_tls: Some(true),
        };

        let merged = Config::merge(base(), env, overrides);

        assert_eq!(
            merged,
            Config {
                host: "base.local".to_string(),
                port: 9090,
                use_tls: true,
            }
        );
    }
}
