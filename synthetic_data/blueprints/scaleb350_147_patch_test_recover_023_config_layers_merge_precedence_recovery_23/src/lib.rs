#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub tls: Option<bool>,
}

impl Config {
    pub fn new(host: Option<&str>, port: Option<u16>, tls: Option<bool>) -> Self {
        Self {
            host: host.map(str::to_string),
            port,
            tls,
        }
    }
}

pub fn merge_config(defaults: &Config, env: &Config, user: &Config) -> Config {
    Config {
        host: defaults.host.clone().or_else(|| env.host.clone()).or_else(|| user.host.clone()),
        port: defaults.port.or(env.port).or(user.port),
        tls: defaults.tls.or(env.tls).or(user.tls),
    }
}

#[cfg(test)]
mod tests {
    use super::{merge_config, Config};

    #[test]
    fn user_values_override_env_and_defaults() {
        let defaults = Config::new(Some("localhost"), Some(80), Some(false));
        let env = Config::new(Some("env.internal"), Some(8080), None);
        let user = Config::new(Some("user.example.com"), None, Some(true));

        let merged = merge_config(&defaults, &env, &user);

        assert_eq!(merged.host.as_deref(), Some("user.example.com"));
        assert_eq!(merged.port, Some(8080));
        assert_eq!(merged.tls, Some(true));
    }

    #[test]
    fn env_values_override_defaults_when_user_missing() {
        let defaults = Config::new(Some("localhost"), Some(80), Some(false));
        let env = Config::new(Some("env.internal"), Some(3000), Some(true));
        let user = Config::new(None, None, None);

        let merged = merge_config(&defaults, &env, &user);

        assert_eq!(merged.host.as_deref(), Some("env.internal"));
        assert_eq!(merged.port, Some(3000));
        assert_eq!(merged.tls, Some(true));
    }

    #[test]
    fn explicit_false_from_higher_precedence_layer_must_win() {
        let defaults = Config::new(None, None, Some(true));
        let env = Config::new(None, None, Some(true));
        let user = Config::new(None, None, Some(false));

        let merged = merge_config(&defaults, &env, &user);

        assert_eq!(merged.tls, Some(false));
    }
}
