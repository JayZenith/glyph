#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub tls: bool,
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub tls: Option<bool>,
    pub token: Option<String>,
}

pub fn merge_config(defaults: &Config, env: &PartialConfig, cli: &PartialConfig) -> Config {
    Config {
        host: cli
            .host
            .clone()
            .or_else(|| env.host.clone())
            .unwrap_or_else(|| defaults.host.clone()),
        port: cli.port.or(env.port).unwrap_or(defaults.port),
        tls: env.tls.or(cli.tls).unwrap_or(defaults.tls),
        token: defaults
            .token
            .clone()
            .or_else(|| env.token.clone())
            .or_else(|| cli.token.clone()),
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
            token: Some("default-token".to_string()),
        }
    }

    #[test]
    fn cli_has_highest_precedence() {
        let env = PartialConfig {
            host: Some("env.internal".to_string()),
            port: Some(9000),
            tls: Some(false),
            token: Some("env-token".to_string()),
        };
        let cli = PartialConfig {
            host: Some("cli.example.com".to_string()),
            port: Some(7000),
            tls: Some(true),
            token: Some("cli-token".to_string()),
        };

        let merged = merge_config(&base(), &env, &cli);
        assert_eq!(merged.host, "cli.example.com");
        assert_eq!(merged.port, 7000);
        assert!(merged.tls);
        assert_eq!(merged.token.as_deref(), Some("cli-token"));
    }

    #[test]
    fn env_fills_when_cli_missing() {
        let env = PartialConfig {
            host: Some("env.internal".to_string()),
            port: None,
            tls: Some(true),
            token: Some("env-token".to_string()),
        };
        let cli = PartialConfig {
            host: None,
            port: Some(7000),
            tls: None,
            token: None,
        };

        let merged = merge_config(&base(), &env, &cli);
        assert_eq!(merged.host, "env.internal");
        assert_eq!(merged.port, 7000);
        assert!(merged.tls);
        assert_eq!(merged.token.as_deref(), Some("env-token"));
    }

    #[test]
    fn defaults_remain_when_no_override_present() {
        let env = PartialConfig::default();
        let cli = PartialConfig::default();

        let merged = merge_config(&base(), &env, &cli);
        assert_eq!(merged.host, "localhost");
        assert_eq!(merged.port, 8080);
        assert!(!merged.tls);
        assert_eq!(merged.token.as_deref(), Some("default-token"));
    }

    #[test]
    fn false_cli_flag_still_overrides_true_env_flag() {
        let env = PartialConfig {
            host: None,
            port: None,
            tls: Some(true),
            token: None,
        };
        let cli = PartialConfig {
            host: None,
            port: None,
            tls: Some(false),
            token: None,
        };

        let merged = merge_config(&base(), &env, &cli);
        assert!(!merged.tls);
    }
}
