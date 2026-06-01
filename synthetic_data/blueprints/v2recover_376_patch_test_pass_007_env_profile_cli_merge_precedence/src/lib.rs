#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub mode: String,
    pub retries: u8,
    pub verbose: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub mode: Option<String>,
    pub retries: Option<u8>,
    pub verbose: Option<bool>,
}

impl PartialConfig {
    fn apply_over(self, mut base: Config) -> Config {
        if let Some(mode) = self.mode {
            base.mode = mode;
        }
        if let Some(retries) = self.retries {
            base.retries = retries;
        }
        if self.verbose.unwrap_or(false) {
            base.verbose = true;
        }
        base
    }
}

pub fn merge_config(
    defaults: Config,
    env: PartialConfig,
    profile: PartialConfig,
    cli: PartialConfig,
) -> Config {
    let merged = profile.apply_over(defaults);
    let merged = env.apply_over(merged);
    cli.apply_over(merged)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            mode: "safe".to_string(),
            retries: 2,
            verbose: true,
        }
    }

    #[test]
    fn precedence_is_defaults_then_env_then_profile_then_cli() {
        let env = PartialConfig {
            mode: Some("env".to_string()),
            retries: Some(4),
            verbose: Some(false),
        };
        let profile = PartialConfig {
            mode: Some("profile".to_string()),
            retries: None,
            verbose: Some(true),
        };
        let cli = PartialConfig {
            mode: None,
            retries: Some(1),
            verbose: None,
        };

        let merged = merge_config(defaults(), env, profile, cli);
        assert_eq!(
            merged,
            Config {
                mode: "profile".to_string(),
                retries: 1,
                verbose: true,
            }
        );
    }

    #[test]
    fn explicit_false_overrides_lower_layers() {
        let env = PartialConfig {
            mode: None,
            retries: None,
            verbose: Some(true),
        };
        let profile = PartialConfig {
            mode: None,
            retries: None,
            verbose: Some(true),
        };
        let cli = PartialConfig {
            mode: None,
            retries: None,
            verbose: Some(false),
        };

        let merged = merge_config(defaults(), env, profile, cli);
        assert!(!merged.verbose);
    }

    #[test]
    fn absent_values_do_not_override() {
        let env = PartialConfig {
            mode: None,
            retries: Some(5),
            verbose: None,
        };
        let profile = PartialConfig::default();
        let cli = PartialConfig::default();

        let merged = merge_config(defaults(), env, profile, cli);
        assert_eq!(merged.mode, "safe");
        assert_eq!(merged.retries, 5);
        assert!(merged.verbose);
    }
}
