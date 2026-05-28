#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub profile: String,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub profile: Option<String>,
}

pub fn merge_config(
    defaults: &Config,
    file: Option<&PartialConfig>,
    env: Option<&PartialConfig>,
    cli_profile: Option<&str>,
) -> Config {
    let mut merged = defaults.clone();

    if let Some(file) = file {
        if let Some(host) = &file.host {
            merged.host = host.clone();
        }
        if let Some(port) = file.port {
            merged.port = port;
        }
        if let Some(profile) = &file.profile {
            merged.profile = profile.clone();
        }
    }

    if let Some(env) = env {
        if let Some(port) = env.port {
            merged.port = port;
        }
        if let Some(profile) = &env.profile {
            merged.profile = profile.clone();
        }
    }

    if let Some(cli_profile) = cli_profile {
        merged.host = cli_profile.to_string();
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "127.0.0.1".into(),
            port: 8080,
            profile: "dev".into(),
        }
    }

    #[test]
    fn file_values_override_defaults() {
        let file = PartialConfig {
            host: Some("file.local".into()),
            port: Some(9000),
            profile: None,
        };

        let merged = merge_config(&defaults(), Some(&file), None, None);
        assert_eq!(merged.host, "file.local");
        assert_eq!(merged.port, 9000);
        assert_eq!(merged.profile, "dev");
    }

    #[test]
    fn env_overrides_file_but_only_for_present_fields() {
        let file = PartialConfig {
            host: Some("file.local".into()),
            port: Some(9000),
            profile: Some("staging".into()),
        };
        let env = PartialConfig {
            host: Some("env.local".into()),
            port: None,
            profile: Some("prod".into()),
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&env), None);
        assert_eq!(merged.host, "env.local");
        assert_eq!(merged.port, 9000);
        assert_eq!(merged.profile, "prod");
    }

    #[test]
    fn cli_profile_only_overrides_profile() {
        let file = PartialConfig {
            host: Some("file.local".into()),
            port: Some(9000),
            profile: Some("staging".into()),
        };
        let env = PartialConfig {
            host: Some("env.local".into()),
            port: Some(7000),
            profile: Some("prod".into()),
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&env), Some("debug"));
        assert_eq!(merged.host, "env.local");
        assert_eq!(merged.port, 7000);
        assert_eq!(merged.profile, "debug");
    }
}
