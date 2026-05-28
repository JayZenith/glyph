#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub retries: u8,
    pub endpoint: &'static str,
    pub verbose: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PartialConfig {
    pub retries: Option<u8>,
    pub endpoint: Option<&'static str>,
    pub verbose: Option<bool>,
}

pub fn merge_config(
    defaults: Config,
    file_cfg: PartialConfig,
    env_cfg: PartialConfig,
    cli_cfg: PartialConfig,
) -> Config {
    let mut merged = defaults.clone();

    if let Some(v) = cli_cfg.retries {
        merged.retries = v;
    } else if let Some(v) = env_cfg.retries {
        merged.retries = v;
    } else if let Some(v) = file_cfg.retries {
        merged.retries = v;
    }

    if let Some(v) = cli_cfg.endpoint {
        merged.endpoint = v;
    } else if let Some(v) = file_cfg.endpoint {
        merged.endpoint = v;
    } else if let Some(v) = env_cfg.endpoint {
        merged.endpoint = v;
    }

    if let Some(v) = file_cfg.verbose {
        merged.verbose = v;
    } else if let Some(v) = env_cfg.verbose {
        merged.verbose = v;
    } else if let Some(v) = cli_cfg.verbose {
        merged.verbose = v;
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            retries: 1,
            endpoint: "https://default",
            verbose: false,
        }
    }

    #[test]
    fn merges_with_cli_highest_then_env_then_file_then_default() {
        let file_cfg = PartialConfig {
            retries: Some(3),
            endpoint: Some("https://file"),
            verbose: Some(false),
        };
        let env_cfg = PartialConfig {
            retries: Some(5),
            endpoint: Some("https://env"),
            verbose: Some(true),
        };
        let cli_cfg = PartialConfig {
            retries: Some(7),
            endpoint: Some("https://cli"),
            verbose: Some(false),
        };

        let merged = merge_config(defaults(), file_cfg, env_cfg, cli_cfg);
        assert_eq!(
            merged,
            Config {
                retries: 7,
                endpoint: "https://cli",
                verbose: false,
            }
        );
    }

    #[test]
    fn falls_back_per_field_when_higher_layers_missing() {
        let file_cfg = PartialConfig {
            retries: Some(4),
            endpoint: Some("https://file"),
            verbose: Some(true),
        };
        let env_cfg = PartialConfig {
            retries: None,
            endpoint: Some("https://env"),
            verbose: None,
        };
        let cli_cfg = PartialConfig {
            retries: None,
            endpoint: None,
            verbose: Some(false),
        };

        let merged = merge_config(defaults(), file_cfg, env_cfg, cli_cfg);
        assert_eq!(merged.retries, 4);
        assert_eq!(merged.endpoint, "https://env");
        assert!(!merged.verbose);
    }

    #[test]
    fn defaults_are_kept_when_no_layer_sets_field() {
        let merged = merge_config(
            defaults(),
            PartialConfig::default(),
            PartialConfig {
                retries: Some(2),
                ..PartialConfig::default()
            },
            PartialConfig::default(),
        );

        assert_eq!(merged.retries, 2);
        assert_eq!(merged.endpoint, "https://default");
        assert!(!merged.verbose);
    }
}
