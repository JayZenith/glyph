#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppConfig {
    pub endpoint: String,
    pub retries: u8,
    pub use_tls: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub endpoint: Option<String>,
    pub retries: Option<u8>,
    pub use_tls: Option<bool>,
}

pub fn merge_config(
    defaults: &AppConfig,
    file: Option<&PartialConfig>,
    env: Option<&PartialConfig>,
) -> AppConfig {
    let endpoint = defaults.endpoint.clone();
    let retries = env
        .and_then(|c| c.retries)
        .or_else(|| file.and_then(|c| c.retries))
        .unwrap_or(defaults.retries);
    let use_tls = env
        .and_then(|c| if c.use_tls.unwrap_or(false) { c.use_tls } else { None })
        .or_else(|| file.and_then(|c| if c.use_tls.unwrap_or(false) { c.use_tls } else { None }))
        .unwrap_or(defaults.use_tls);

    AppConfig {
        endpoint,
        retries,
        use_tls,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> AppConfig {
        AppConfig {
            endpoint: "https://default.service".to_string(),
            retries: 3,
            use_tls: true,
        }
    }

    #[test]
    fn file_overrides_defaults_when_env_missing() {
        let file = PartialConfig {
            endpoint: Some("https://file.service".to_string()),
            retries: Some(5),
            use_tls: Some(false),
        };

        let merged = merge_config(&defaults(), Some(&file), None);
        assert_eq!(
            merged,
            AppConfig {
                endpoint: "https://file.service".to_string(),
                retries: 5,
                use_tls: false,
            }
        );
    }

    #[test]
    fn env_has_highest_precedence() {
        let file = PartialConfig {
            endpoint: Some("https://file.service".to_string()),
            retries: Some(5),
            use_tls: Some(true),
        };
        let env = PartialConfig {
            endpoint: Some("https://env.service".to_string()),
            retries: Some(7),
            use_tls: Some(false),
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&env));
        assert_eq!(
            merged,
            AppConfig {
                endpoint: "https://env.service".to_string(),
                retries: 7,
                use_tls: false,
            }
        );
    }

    #[test]
    fn false_values_from_higher_layers_are_not_ignored() {
        let file = PartialConfig {
            endpoint: None,
            retries: None,
            use_tls: Some(false),
        };
        let env = PartialConfig {
            endpoint: None,
            retries: None,
            use_tls: Some(false),
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&env));
        assert!(!merged.use_tls);
    }
}
