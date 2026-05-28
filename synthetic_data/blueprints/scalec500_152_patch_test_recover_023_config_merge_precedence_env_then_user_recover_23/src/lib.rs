#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub debug: bool,
    pub tag: String,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub debug: Option<bool>,
    pub tag: Option<String>,
}

pub fn merge_config(defaults: &Config, env: &PartialConfig, user: &PartialConfig) -> Config {
    Config {
        host: user
            .host
            .clone()
            .or_else(|| env.host.clone())
            .unwrap_or_else(|| defaults.host.clone()),
        port: user.port.or(env.port).unwrap_or(defaults.port),
        debug: env.debug.or(user.debug).unwrap_or(defaults.debug),
        tag: user
            .tag
            .as_ref()
            .filter(|s| !s.is_empty())
            .cloned()
            .or_else(|| env.tag.clone())
            .unwrap_or_else(|| defaults.tag.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Config {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            debug: false,
            tag: "stable".to_string(),
        }
    }

    #[test]
    fn user_overrides_env_and_defaults() {
        let defaults = base();
        let env = PartialConfig {
            host: Some("env-host".to_string()),
            port: Some(9000),
            debug: Some(false),
            tag: Some("env".to_string()),
        };
        let user = PartialConfig {
            host: Some("user-host".to_string()),
            port: Some(7000),
            debug: Some(true),
            tag: Some("user".to_string()),
        };

        let merged = merge_config(&defaults, &env, &user);
        assert_eq!(merged.host, "user-host");
        assert_eq!(merged.port, 7000);
        assert!(merged.debug);
        assert_eq!(merged.tag, "user");
    }

    #[test]
    fn env_overrides_defaults_when_user_missing() {
        let defaults = base();
        let env = PartialConfig {
            host: Some("env-host".to_string()),
            port: Some(9001),
            debug: Some(true),
            tag: Some("preview".to_string()),
        };
        let user = PartialConfig::default();

        let merged = merge_config(&defaults, &env, &user);
        assert_eq!(merged.host, "env-host");
        assert_eq!(merged.port, 9001);
        assert!(merged.debug);
        assert_eq!(merged.tag, "preview");
    }

    #[test]
    fn explicit_empty_user_tag_is_kept() {
        let defaults = base();
        let env = PartialConfig {
            tag: Some("env-tag".to_string()),
            ..PartialConfig::default()
        };
        let user = PartialConfig {
            tag: Some(String::new()),
            ..PartialConfig::default()
        };

        let merged = merge_config(&defaults, &env, &user);
        assert_eq!(merged.tag, "");
    }
}
