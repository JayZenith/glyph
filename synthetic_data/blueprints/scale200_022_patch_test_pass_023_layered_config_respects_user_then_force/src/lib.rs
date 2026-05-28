#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub tls: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub tls: Option<bool>,
}

pub fn merge_config(
    defaults: &Config,
    env: &PartialConfig,
    user: &PartialConfig,
    force_tls: Option<bool>,
) -> Config {
    Config {
        host: env.host.clone().or_else(|| user.host.clone()).unwrap_or_else(|| defaults.host.clone()),
        port: env.port.or(user.port).unwrap_or(defaults.port),
        tls: env.tls.or(user.tls).or(force_tls).unwrap_or(defaults.tls),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Config {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            tls: false,
        }
    }

    #[test]
    fn user_overrides_env_for_host_and_port() {
        let env = PartialConfig {
            host: Some("env.internal".to_string()),
            port: Some(9000),
            tls: None,
        };
        let user = PartialConfig {
            host: Some("user.example.com".to_string()),
            port: Some(7000),
            tls: None,
        };

        let merged = merge_config(&base(), &env, &user, None);
        assert_eq!(merged.host, "user.example.com");
        assert_eq!(merged.port, 7000);
    }

    #[test]
    fn force_tls_has_highest_precedence_even_over_explicit_false() {
        let env = PartialConfig {
            host: None,
            port: None,
            tls: Some(false),
        };
        let user = PartialConfig {
            host: None,
            port: None,
            tls: Some(false),
        };

        let merged = merge_config(&base(), &env, &user, Some(true));
        assert!(merged.tls);
    }

    #[test]
    fn falls_back_per_field_when_layers_are_missing() {
        let env = PartialConfig {
            host: None,
            port: Some(9090),
            tls: None,
        };
        let user = PartialConfig {
            host: Some("app.local".to_string()),
            port: None,
            tls: None,
        };

        let merged = merge_config(&base(), &env, &user, None);
        assert_eq!(merged.host, "app.local");
        assert_eq!(merged.port, 9090);
        assert!(!merged.tls);
    }
}
