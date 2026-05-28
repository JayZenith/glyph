#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub token: String,
    pub use_tls: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub token: Option<String>,
    pub use_tls: Option<bool>,
}

pub fn merge_config(
    defaults: &Config,
    file: &PartialConfig,
    env: &PartialConfig,
    override_cfg: &PartialConfig,
) -> Config {
    Config {
        host: override_cfg
            .host
            .clone()
            .or_else(|| env.host.clone())
            .or_else(|| file.host.clone())
            .unwrap_or_else(|| defaults.host.clone()),
        port: override_cfg
            .port
            .or(env.port)
            .or(file.port)
            .unwrap_or(defaults.port),
        token: file
            .token
            .clone()
            .or_else(|| env.token.clone())
            .or_else(|| override_cfg.token.clone())
            .unwrap_or_else(|| defaults.token.clone()),
        use_tls: file
            .use_tls
            .or(env.use_tls)
            .or(override_cfg.use_tls)
            .unwrap_or(defaults.use_tls),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "localhost".into(),
            port: 8080,
            token: "default-token".into(),
            use_tls: false,
        }
    }

    #[test]
    fn precedence_is_override_then_env_then_file_then_defaults() {
        let file = PartialConfig {
            host: Some("file-host".into()),
            port: Some(7000),
            token: Some("file-token".into()),
            use_tls: Some(false),
        };
        let env = PartialConfig {
            host: Some("env-host".into()),
            port: Some(8000),
            token: Some("env-token".into()),
            use_tls: Some(true),
        };
        let override_cfg = PartialConfig {
            host: Some("override-host".into()),
            port: Some(9000),
            token: Some("override-token".into()),
            use_tls: Some(false),
        };

        let merged = merge_config(&defaults(), &file, &env, &override_cfg);
        assert_eq!(merged.host, "override-host");
        assert_eq!(merged.port, 9000);
        assert_eq!(merged.token, "override-token");
        assert!(!merged.use_tls);
    }

    #[test]
    fn env_beats_file_and_empty_strings_are_valid_values() {
        let file = PartialConfig {
            host: Some("file-host".into()),
            port: Some(7000),
            token: Some("file-token".into()),
            use_tls: Some(false),
        };
        let env = PartialConfig {
            host: Some(String::new()),
            port: None,
            token: Some(String::new()),
            use_tls: Some(true),
        };
        let override_cfg = PartialConfig::default();

        let merged = merge_config(&defaults(), &file, &env, &override_cfg);
        assert_eq!(merged.host, "");
        assert_eq!(merged.port, 7000);
        assert_eq!(merged.token, "");
        assert!(merged.use_tls);
    }

    #[test]
    fn defaults_fill_only_missing_values() {
        let file = PartialConfig {
            host: None,
            port: Some(3000),
            token: None,
            use_tls: None,
        };
        let env = PartialConfig {
            host: Some("env-host".into()),
            port: None,
            token: None,
            use_tls: Some(true),
        };
        let override_cfg = PartialConfig::default();

        let merged = merge_config(&defaults(), &file, &env, &override_cfg);
        assert_eq!(merged.host, "env-host");
        assert_eq!(merged.port, 3000);
        assert_eq!(merged.token, "default-token");
        assert!(merged.use_tls);
    }
}
