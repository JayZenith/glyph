#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub endpoint: String,
    pub timeout_secs: u32,
    pub use_tls: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub endpoint: Option<String>,
    pub timeout_secs: Option<u32>,
    pub use_tls: Option<bool>,
}

pub fn merge_config(defaults: &Config, file: &PartialConfig, env: &PartialConfig) -> Config {
    Config {
        endpoint: env
            .endpoint
            .clone()
            .or_else(|| file.endpoint.clone())
            .unwrap_or_else(|| defaults.endpoint.clone()),
        timeout_secs: env
            .timeout_secs
            .or(file.timeout_secs)
            .unwrap_or(defaults.timeout_secs),
        use_tls: env.use_tls.unwrap_or(file.use_tls.unwrap_or(defaults.use_tls)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            endpoint: "https://default.service".to_string(),
            timeout_secs: 30,
            use_tls: true,
        }
    }

    #[test]
    fn file_values_override_defaults() {
        let file = PartialConfig {
            endpoint: Some("https://file.service".to_string()),
            timeout_secs: Some(10),
            use_tls: Some(false),
        };
        let env = PartialConfig::default();

        let merged = merge_config(&defaults(), &file, &env);

        assert_eq!(merged.endpoint, "https://file.service");
        assert_eq!(merged.timeout_secs, 10);
        assert!(!merged.use_tls);
    }

    #[test]
    fn env_values_override_file_values() {
        let file = PartialConfig {
            endpoint: Some("https://file.service".to_string()),
            timeout_secs: Some(10),
            use_tls: Some(true),
        };
        let env = PartialConfig {
            endpoint: Some("https://env.service".to_string()),
            timeout_secs: Some(5),
            use_tls: Some(false),
        };

        let merged = merge_config(&defaults(), &file, &env);

        assert_eq!(merged.endpoint, "https://env.service");
        assert_eq!(merged.timeout_secs, 5);
        assert!(!merged.use_tls);
    }

    #[test]
    fn missing_values_fall_back_by_precedence() {
        let file = PartialConfig {
            endpoint: None,
            timeout_secs: Some(45),
            use_tls: None,
        };
        let env = PartialConfig {
            endpoint: Some("https://env.service".to_string()),
            timeout_secs: None,
            use_tls: None,
        };

        let merged = merge_config(&defaults(), &file, &env);

        assert_eq!(merged.endpoint, "https://env.service");
        assert_eq!(merged.timeout_secs, 45);
        assert!(merged.use_tls);
    }

    #[test]
    fn explicit_false_from_file_must_not_fall_back_to_default_true() {
        let file = PartialConfig {
            endpoint: None,
            timeout_secs: None,
            use_tls: Some(false),
        };
        let env = PartialConfig::default();

        let merged = merge_config(&defaults(), &file, &env);

        assert!(!merged.use_tls);
    }
}
