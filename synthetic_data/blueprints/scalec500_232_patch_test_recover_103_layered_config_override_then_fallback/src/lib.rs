#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Config {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub profile: Option<String>,
}

pub fn merge_config(defaults: &Config, env: &Config, overrides: &Config, fallback_host: &str) -> Config {
    let host = defaults
        .host
        .clone()
        .or_else(|| env.host.clone())
        .or_else(|| overrides.host.clone())
        .or_else(|| Some(fallback_host.to_string()));

    let port = defaults.port.or(env.port).or(overrides.port);

    let profile = overrides
        .profile
        .clone()
        .or_else(|| defaults.profile.clone())
        .or_else(|| env.profile.clone());

    Config { host, port, profile }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn override_beats_env_and_default_for_host_and_port() {
        let defaults = Config {
            host: Some("default.local".into()),
            port: Some(8080),
            profile: Some("dev".into()),
        };
        let env = Config {
            host: Some("env.local".into()),
            port: Some(9000),
            profile: Some("staging".into()),
        };
        let overrides = Config {
            host: Some("cli.local".into()),
            port: Some(7000),
            profile: None,
        };

        let merged = merge_config(&defaults, &env, &overrides, "fallback.local");
        assert_eq!(merged.host.as_deref(), Some("cli.local"));
        assert_eq!(merged.port, Some(7000));
        assert_eq!(merged.profile.as_deref(), Some("dev"));
    }

    #[test]
    fn env_beats_default_when_override_missing() {
        let defaults = Config {
            host: Some("default.local".into()),
            port: Some(8080),
            profile: Some("dev".into()),
        };
        let env = Config {
            host: Some("env.local".into()),
            port: Some(9000),
            profile: Some("staging".into()),
        };
        let overrides = Config::default();

        let merged = merge_config(&defaults, &env, &overrides, "fallback.local");
        assert_eq!(merged.host.as_deref(), Some("env.local"));
        assert_eq!(merged.port, Some(9000));
        assert_eq!(merged.profile.as_deref(), Some("dev"));
    }

    #[test]
    fn fallback_host_only_used_when_all_sources_missing() {
        let merged = merge_config(
            &Config::default(),
            &Config::default(),
            &Config::default(),
            "fallback.local",
        );
        assert_eq!(merged.host.as_deref(), Some("fallback.local"));
        assert_eq!(merged.port, None);
        assert_eq!(merged.profile, None);
    }

    #[test]
    fn env_profile_used_when_default_and_override_missing() {
        let defaults = Config::default();
        let env = Config {
            host: None,
            port: None,
            profile: Some("prod".into()),
        };
        let overrides = Config::default();

        let merged = merge_config(&defaults, &env, &overrides, "fallback.local");
        assert_eq!(merged.profile.as_deref(), Some("prod"));
    }
}
