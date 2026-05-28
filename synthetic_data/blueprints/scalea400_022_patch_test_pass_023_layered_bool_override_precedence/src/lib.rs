#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub retries: u8,
    pub endpoint: String,
    pub dry_run: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub retries: Option<u8>,
    pub endpoint: Option<String>,
    pub dry_run: Option<bool>,
}

pub fn merge_config(
    defaults: &Config,
    file: Option<&PartialConfig>,
    env: Option<&PartialConfig>,
) -> Config {
    let mut merged = defaults.clone();

    if let Some(file) = file {
        if let Some(retries) = file.retries {
            merged.retries = retries;
        }
        if let Some(endpoint) = &file.endpoint {
            merged.endpoint = endpoint.clone();
        }
        if let Some(dry_run) = file.dry_run {
            merged.dry_run = dry_run;
        }
    }

    if let Some(env) = env {
        if let Some(retries) = env.retries {
            merged.retries = retries;
        }
        if let Some(endpoint) = &env.endpoint {
            merged.endpoint = endpoint.clone();
        }
        if let Some(dry_run) = env.dry_run {
            if dry_run {
                merged.dry_run = true;
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
            retries: 3,
            endpoint: "https://default.service".into(),
            dry_run: false,
        }
    }

    #[test]
    fn env_overrides_file_values() {
        let file = PartialConfig {
            retries: Some(5),
            endpoint: Some("https://file.service".into()),
            dry_run: Some(true),
        };
        let env = PartialConfig {
            retries: Some(1),
            endpoint: Some("https://env.service".into()),
            dry_run: Some(false),
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&env));

        assert_eq!(
            merged,
            Config {
                retries: 1,
                endpoint: "https://env.service".into(),
                dry_run: false,
            }
        );
    }

    #[test]
    fn file_fills_missing_env_fields() {
        let file = PartialConfig {
            retries: Some(4),
            endpoint: Some("https://file.service".into()),
            dry_run: Some(true),
        };
        let env = PartialConfig {
            retries: None,
            endpoint: None,
            dry_run: None,
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&env));

        assert_eq!(
            merged,
            Config {
                retries: 4,
                endpoint: "https://file.service".into(),
                dry_run: true,
            }
        );
    }

    #[test]
    fn defaults_are_used_when_layers_are_missing() {
        let merged = merge_config(&defaults(), None, None);
        assert_eq!(merged, defaults());
    }

    #[test]
    fn env_can_disable_file_enabled_dry_run() {
        let file = PartialConfig {
            retries: None,
            endpoint: None,
            dry_run: Some(true),
        };
        let env = PartialConfig {
            retries: None,
            endpoint: None,
            dry_run: Some(false),
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&env));
        assert!(!merged.dry_run);
    }
}
