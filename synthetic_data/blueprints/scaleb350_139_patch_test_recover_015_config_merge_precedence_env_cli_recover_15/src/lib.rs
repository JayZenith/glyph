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

pub fn merge_config(defaults: &Config, env: &PartialConfig, cli: &PartialConfig) -> Config {
    let endpoint = defaults
        .endpoint
        .clone();

    let retries = cli
        .retries
        .or(env.retries)
        .unwrap_or(defaults.retries);

    let verbose = cli
        .verbose
        .unwrap_or(defaults.verbose);

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
            retries: 2,
            verbose: true,
        }
    }

    #[test]
    fn env_overrides_defaults_when_cli_missing() {
        let env = PartialConfig {
            endpoint: Some("https://env.service".to_string()),
            retries: Some(5),
            verbose: Some(false),
        };
        let cli = PartialConfig::default();

        let merged = merge_config(&defaults(), &env, &cli);

        assert_eq!(
            merged,
            Config {
                endpoint: "https://env.service".to_string(),
                retries: 5,
                verbose: false,
            }
        );
    }

    #[test]
    fn cli_has_highest_precedence() {
        let env = PartialConfig {
            endpoint: Some("https://env.service".to_string()),
            retries: Some(5),
            verbose: Some(true),
        };
        let cli = PartialConfig {
            endpoint: Some("https://cli.service".to_string()),
            retries: Some(1),
            verbose: Some(false),
        };

        let merged = merge_config(&defaults(), &env, &cli);

        assert_eq!(
            merged,
            Config {
                endpoint: "https://cli.service".to_string(),
                retries: 1,
                verbose: false,
            }
        );
    }

    #[test]
    fn explicit_false_from_cli_must_override_env_true() {
        let env = PartialConfig {
            endpoint: None,
            retries: None,
            verbose: Some(true),
        };
        let cli = PartialConfig {
            endpoint: None,
            retries: None,
            verbose: Some(false),
        };

        let merged = merge_config(&defaults(), &env, &cli);
        assert!(!merged.verbose);
    }
}
