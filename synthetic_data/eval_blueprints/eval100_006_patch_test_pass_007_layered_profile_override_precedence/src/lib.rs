#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub retries: u8,
    pub timeout_ms: u32,
    pub verbose: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub retries: Option<u8>,
    pub timeout_ms: Option<u32>,
    pub verbose: Option<bool>,
}

pub fn merge_config(
    defaults: &Config,
    profile: Option<&PartialConfig>,
    override_cfg: Option<&PartialConfig>,
) -> Config {
    let profile = profile.cloned().unwrap_or_default();
    let override_cfg = override_cfg.cloned().unwrap_or_default();

    Config {
        retries: override_cfg
            .retries
            .or(defaults.retries.into())
            .or(profile.retries)
            .unwrap(),
        timeout_ms: override_cfg
            .timeout_ms
            .or(defaults.timeout_ms.into())
            .or(profile.timeout_ms)
            .unwrap(),
        verbose: override_cfg.verbose.unwrap_or(profile.verbose.unwrap_or(defaults.verbose)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Config {
        Config {
            retries: 3,
            timeout_ms: 1000,
            verbose: false,
        }
    }

    #[test]
    fn uses_defaults_when_no_layers_present() {
        let merged = merge_config(&base(), None, None);
        assert_eq!(
            merged,
            Config {
                retries: 3,
                timeout_ms: 1000,
                verbose: false,
            }
        );
    }

    #[test]
    fn profile_overrides_defaults() {
        let profile = PartialConfig {
            retries: Some(5),
            timeout_ms: Some(2500),
            verbose: Some(true),
        };

        let merged = merge_config(&base(), Some(&profile), None);
        assert_eq!(
            merged,
            Config {
                retries: 5,
                timeout_ms: 2500,
                verbose: true,
            }
        );
    }

    #[test]
    fn explicit_override_beats_profile_and_defaults() {
        let profile = PartialConfig {
            retries: Some(5),
            timeout_ms: Some(2500),
            verbose: Some(true),
        };
        let override_cfg = PartialConfig {
            retries: Some(1),
            timeout_ms: None,
            verbose: Some(false),
        };

        let merged = merge_config(&base(), Some(&profile), Some(&override_cfg));
        assert_eq!(
            merged,
            Config {
                retries: 1,
                timeout_ms: 2500,
                verbose: false,
            }
        );
    }

    #[test]
    fn lower_layers_fill_missing_override_fields() {
        let profile = PartialConfig {
            retries: None,
            timeout_ms: Some(1800),
            verbose: None,
        };
        let override_cfg = PartialConfig {
            retries: Some(7),
            timeout_ms: None,
            verbose: None,
        };

        let merged = merge_config(&base(), Some(&profile), Some(&override_cfg));
        assert_eq!(
            merged,
            Config {
                retries: 7,
                timeout_ms: 1800,
                verbose: false,
            }
        );
    }
}
