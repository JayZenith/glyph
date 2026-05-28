#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveConfig {
    pub host: String,
    pub port: u16,
    pub use_tls: bool,
    pub retries: u8,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub use_tls: Option<bool>,
    pub retries: Option<u8>,
}

pub fn merge_config(
    defaults: &EffectiveConfig,
    file: &PartialConfig,
    env: &PartialConfig,
) -> EffectiveConfig {
    EffectiveConfig {
        host: file
            .host
            .clone()
            .or_else(|| env.host.clone())
            .unwrap_or_else(|| defaults.host.clone()),
        port: file.port.or(env.port).unwrap_or(defaults.port),
        use_tls: file.use_tls.unwrap_or(env.use_tls.unwrap_or(defaults.use_tls)),
        retries: env.retries.or(file.retries).unwrap_or(defaults.retries),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> EffectiveConfig {
        EffectiveConfig {
            host: "localhost".to_string(),
            port: 8080,
            use_tls: true,
            retries: 3,
        }
    }

    #[test]
    fn env_overrides_file_for_host_and_port() {
        let file = PartialConfig {
            host: Some("file.internal".to_string()),
            port: Some(9000),
            use_tls: None,
            retries: None,
        };
        let env = PartialConfig {
            host: Some("env.internal".to_string()),
            port: Some(7000),
            use_tls: None,
            retries: None,
        };

        let merged = merge_config(&defaults(), &file, &env);
        assert_eq!(merged.host, "env.internal");
        assert_eq!(merged.port, 7000);
    }

    #[test]
    fn explicit_false_from_env_must_override_true_from_file() {
        let file = PartialConfig {
            host: None,
            port: None,
            use_tls: Some(true),
            retries: None,
        };
        let env = PartialConfig {
            host: None,
            port: None,
            use_tls: Some(false),
            retries: None,
        };

        let merged = merge_config(&defaults(), &file, &env);
        assert!(!merged.use_tls);
    }

    #[test]
    fn retries_fall_back_to_file_then_defaults() {
        let file = PartialConfig {
            host: None,
            port: None,
            use_tls: None,
            retries: Some(5),
        };
        let env = PartialConfig::default();

        let merged = merge_config(&defaults(), &file, &env);
        assert_eq!(merged.retries, 5);

        let merged2 = merge_config(&defaults(), &PartialConfig::default(), &PartialConfig::default());
        assert_eq!(merged2.retries, 3);
    }
}
