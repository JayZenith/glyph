#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub endpoint: String,
    pub profile: String,
    pub retries: u8,
    pub color: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub endpoint: Option<String>,
    pub profile: Option<String>,
    pub retries: Option<u8>,
    pub color: Option<bool>,
}

pub fn merge_config(base: &Config, env: &PartialConfig, cli: &PartialConfig) -> Config {
    Config {
        endpoint: cli
            .endpoint
            .clone()
            .or_else(|| env.endpoint.clone())
            .unwrap_or_else(|| base.endpoint.clone()),
        profile: cli
            .profile
            .clone()
            .or_else(|| env.profile.clone())
            .unwrap_or_else(|| base.profile.clone()),
        retries: cli.retries.or(env.retries).unwrap_or(base.retries),
        color: cli.color.unwrap_or(env.color.unwrap_or(true)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Config {
        Config {
            endpoint: "https://default.internal".into(),
            profile: "dev".into(),
            retries: 3,
            color: true,
        }
    }

    #[test]
    fn cli_values_override_env_and_base() {
        let env = PartialConfig {
            endpoint: Some("https://env.internal".into()),
            profile: Some("staging".into()),
            retries: Some(5),
            color: Some(true),
        };
        let cli = PartialConfig {
            endpoint: Some("https://cli.internal".into()),
            profile: Some("prod".into()),
            retries: Some(1),
            color: Some(false),
        };

        let merged = merge_config(&base(), &env, &cli);
        assert_eq!(merged.endpoint, "https://cli.internal");
        assert_eq!(merged.profile, "prod");
        assert_eq!(merged.retries, 1);
        assert!(!merged.color);
    }

    #[test]
    fn empty_string_overrides_are_ignored() {
        let env = PartialConfig {
            endpoint: Some("".into()),
            profile: Some("qa".into()),
            retries: None,
            color: None,
        };
        let cli = PartialConfig {
            endpoint: None,
            profile: Some("".into()),
            retries: None,
            color: None,
        };

        let merged = merge_config(&base(), &env, &cli);
        assert_eq!(merged.endpoint, "https://default.internal");
        assert_eq!(merged.profile, "qa");
    }

    #[test]
    fn explicit_false_from_env_is_preserved_when_cli_missing() {
        let env = PartialConfig {
            endpoint: None,
            profile: None,
            retries: None,
            color: Some(false),
        };
        let cli = PartialConfig::default();

        let merged = merge_config(&base(), &env, &cli);
        assert!(!merged.color);
    }

    #[test]
    fn base_values_are_used_when_no_layer_sets_field() {
        let env = PartialConfig::default();
        let cli = PartialConfig::default();

        let merged = merge_config(&base(), &env, &cli);
        assert_eq!(merged, base());
    }
}
