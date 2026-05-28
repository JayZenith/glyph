#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub tls: bool,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub tls: Option<bool>,
    pub timeout_ms: Option<u64>,
}

pub fn merge_config(defaults: &Config, env: &PartialConfig, user: &PartialConfig) -> Config {
    Config {
        host: env.host.clone().or(user.host.clone()).unwrap_or_else(|| defaults.host.clone()),
        port: user.port.or(env.port).unwrap_or(defaults.port),
        tls: user.tls.or(env.tls).unwrap_or(defaults.tls),
        timeout_ms: env.timeout_ms.or(user.timeout_ms).unwrap_or(defaults.timeout_ms),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "localhost".into(),
            port: 8080,
            tls: false,
            timeout_ms: 1000,
        }
    }

    #[test]
    fn user_overrides_env_and_defaults() {
        let env = PartialConfig {
            host: Some("env.internal".into()),
            port: Some(9000),
            tls: Some(false),
            timeout_ms: Some(2000),
        };
        let user = PartialConfig {
            host: Some("user.example.com".into()),
            port: Some(7000),
            tls: Some(true),
            timeout_ms: Some(5000),
        };

        let merged = merge_config(&defaults(), &env, &user);
        assert_eq!(
            merged,
            Config {
                host: "user.example.com".into(),
                port: 7000,
                tls: true,
                timeout_ms: 5000,
            }
        );
    }

    #[test]
    fn env_used_when_user_missing() {
        let env = PartialConfig {
            host: Some("env.internal".into()),
            port: Some(9000),
            tls: Some(true),
            timeout_ms: Some(2000),
        };
        let user = PartialConfig {
            host: None,
            port: None,
            tls: None,
            timeout_ms: None,
        };

        let merged = merge_config(&defaults(), &env, &user);
        assert_eq!(merged.host, "env.internal");
        assert_eq!(merged.port, 9000);
        assert_eq!(merged.tls, true);
        assert_eq!(merged.timeout_ms, 2000);
    }

    #[test]
    fn defaults_fill_remaining_gaps() {
        let env = PartialConfig {
            host: None,
            port: Some(9090),
            tls: None,
            timeout_ms: None,
        };
        let user = PartialConfig {
            host: None,
            port: None,
            tls: Some(true),
            timeout_ms: None,
        };

        let merged = merge_config(&defaults(), &env, &user);
        assert_eq!(merged.host, "localhost");
        assert_eq!(merged.port, 9090);
        assert_eq!(merged.tls, true);
        assert_eq!(merged.timeout_ms, 1000);
    }
}
