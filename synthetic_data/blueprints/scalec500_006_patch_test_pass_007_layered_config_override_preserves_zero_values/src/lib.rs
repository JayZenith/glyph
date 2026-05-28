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

pub fn merge_config(defaults: &Config, file: &PartialConfig, env: &PartialConfig) -> Config {
    let endpoint = env
        .endpoint
        .clone()
        .or_else(|| file.endpoint.clone())
        .unwrap_or_else(|| defaults.endpoint.clone());

    let retries = env
        .retries
        .filter(|v| *v > 0)
        .or(file.retries.filter(|v| *v > 0))
        .unwrap_or(defaults.retries);

    let verbose = if env.verbose == Some(true) {
        true
    } else if file.verbose == Some(true) {
        true
    } else {
        defaults.verbose
    };

    Config {
        endpoint,
        retries,
        verbose,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            endpoint: "https://default.service".to_string(),
            retries: 3,
            verbose: true,
        }
    }

    #[test]
    fn env_has_highest_precedence_for_endpoint() {
        let file = PartialConfig {
            endpoint: Some("https://file.service".to_string()),
            retries: None,
            verbose: None,
        };
        let env = PartialConfig {
            endpoint: Some("https://env.service".to_string()),
            retries: None,
            verbose: None,
        };

        let merged = merge_config(&defaults(), &file, &env);
        assert_eq!(merged.endpoint, "https://env.service");
    }

    #[test]
    fn file_values_are_used_when_env_missing() {
        let file = PartialConfig {
            endpoint: Some("https://file.service".to_string()),
            retries: Some(5),
            verbose: Some(true),
        };
        let env = PartialConfig::default();

        let merged = merge_config(&defaults(), &file, &env);
        assert_eq!(merged.endpoint, "https://file.service");
        assert_eq!(merged.retries, 5);
        assert!(merged.verbose);
    }

    #[test]
    fn explicit_zero_and_false_from_env_override_lower_layers() {
        let file = PartialConfig {
            endpoint: None,
            retries: Some(7),
            verbose: Some(true),
        };
        let env = PartialConfig {
            endpoint: None,
            retries: Some(0),
            verbose: Some(false),
        };

        let merged = merge_config(&defaults(), &file, &env);
        assert_eq!(merged.retries, 0);
        assert!(!merged.verbose);
    }

    #[test]
    fn explicit_zero_and_false_from_file_override_defaults() {
        let file = PartialConfig {
            endpoint: None,
            retries: Some(0),
            verbose: Some(false),
        };
        let env = PartialConfig::default();

        let merged = merge_config(&defaults(), &file, &env);
        assert_eq!(merged.retries, 0);
        assert!(!merged.verbose);
    }
}
