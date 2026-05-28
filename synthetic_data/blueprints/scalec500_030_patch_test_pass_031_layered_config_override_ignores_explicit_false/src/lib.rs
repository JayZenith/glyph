#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub endpoint: String,
    pub retries: u8,
    pub verbose: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub endpoint: Option<String>,
    pub retries: Option<u8>,
    pub verbose: Option<bool>,
}

pub fn merge_config(
    defaults: &Config,
    file: Option<&PartialConfig>,
    env: Option<&PartialConfig>,
) -> Config {
    let mut merged = defaults.clone();

    if let Some(file_cfg) = file {
        if let Some(endpoint) = &file_cfg.endpoint {
            merged.endpoint = endpoint.clone();
        }
        if let Some(retries) = file_cfg.retries {
            merged.retries = retries;
        }
        if file_cfg.verbose.unwrap_or(false) {
            merged.verbose = true;
        }
    }

    if let Some(env_cfg) = env {
        if let Some(endpoint) = &env_cfg.endpoint {
            merged.endpoint = endpoint.clone();
        }
        if let Some(retries) = env_cfg.retries {
            merged.retries = retries;
        }
        if env_cfg.verbose.unwrap_or(false) {
            merged.verbose = true;
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Config {
        Config {
            endpoint: "https://default.service".to_string(),
            retries: 3,
            verbose: true,
        }
    }

    #[test]
    fn file_overrides_defaults() {
        let file = PartialConfig {
            endpoint: Some("https://file.service".to_string()),
            retries: Some(5),
            verbose: Some(false),
        };

        let merged = merge_config(&base(), Some(&file), None);
        assert_eq!(
            merged,
            Config {
                endpoint: "https://file.service".to_string(),
                retries: 5,
                verbose: false,
            }
        );
    }

    #[test]
    fn env_overrides_file() {
        let file = PartialConfig {
            endpoint: Some("https://file.service".to_string()),
            retries: Some(5),
            verbose: Some(true),
        };
        let env = PartialConfig {
            endpoint: Some("https://env.service".to_string()),
            retries: Some(1),
            verbose: Some(false),
        };

        let merged = merge_config(&base(), Some(&file), Some(&env));
        assert_eq!(
            merged,
            Config {
                endpoint: "https://env.service".to_string(),
                retries: 1,
                verbose: false,
            }
        );
    }

    #[test]
    fn missing_values_fall_back_per_field() {
        let file = PartialConfig {
            endpoint: None,
            retries: Some(7),
            verbose: None,
        };
        let env = PartialConfig {
            endpoint: Some("https://env.service".to_string()),
            retries: None,
            verbose: None,
        };

        let merged = merge_config(&base(), Some(&file), Some(&env));
        assert_eq!(
            merged,
            Config {
                endpoint: "https://env.service".to_string(),
                retries: 7,
                verbose: true,
            }
        );
    }
}
