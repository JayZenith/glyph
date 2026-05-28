#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub tls: bool,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub tls: Option<bool>,
    pub timeout_ms: Option<Option<u64>>,
}

pub fn merge_config(defaults: Config, env: PartialConfig, cli: PartialConfig) -> Config {
    let host = cli.host.or(env.host).unwrap_or(defaults.host);
    let port = cli.port.or(env.port).unwrap_or(defaults.port);
    let tls = cli.tls.or(env.tls).unwrap_or(defaults.tls);
    let timeout_ms = cli.timeout_ms.or(env.timeout_ms).unwrap_or(defaults.timeout_ms);

    Config {
        host,
        port,
        tls,
        timeout_ms,
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
            timeout_ms: Some(5000),
        }
    }

    #[test]
    fn cli_overrides_env_and_defaults() {
        let env = PartialConfig {
            host: Some("env.internal".into()),
            port: Some(9000),
            tls: Some(true),
            timeout_ms: Some(Some(3000)),
        };
        let cli = PartialConfig {
            host: Some("cli.example.com".into()),
            port: Some(7000),
            tls: Some(false),
            timeout_ms: Some(Some(1000)),
        };

        let merged = merge_config(defaults(), env, cli);
        assert_eq!(
            merged,
            Config {
                host: "cli.example.com".into(),
                port: 7000,
                tls: false,
                timeout_ms: Some(1000),
            }
        );
    }

    #[test]
    fn env_fills_missing_cli_values() {
        let env = PartialConfig {
            host: Some("env.internal".into()),
            port: Some(9000),
            tls: Some(true),
            timeout_ms: Some(Some(3000)),
        };
        let cli = PartialConfig {
            host: None,
            port: None,
            tls: Some(false),
            timeout_ms: None,
        };

        let merged = merge_config(defaults(), env, cli);
        assert_eq!(merged.host, "env.internal");
        assert_eq!(merged.port, 9000);
        assert!(!merged.tls);
        assert_eq!(merged.timeout_ms, Some(3000));
    }

    #[test]
    fn missing_timeout_override_keeps_base_value() {
        let env = PartialConfig {
            host: None,
            port: None,
            tls: None,
            timeout_ms: None,
        };
        let cli = PartialConfig {
            host: None,
            port: None,
            tls: None,
            timeout_ms: None,
        };

        let merged = merge_config(defaults(), env, cli);
        assert_eq!(merged.timeout_ms, Some(5000));
    }

    #[test]
    fn explicit_none_timeout_disables_timeout() {
        let env = PartialConfig {
            host: None,
            port: None,
            tls: None,
            timeout_ms: Some(None),
        };
        let cli = PartialConfig::default();

        let merged = merge_config(defaults(), env, cli);
        assert_eq!(merged.timeout_ms, None);
    }

    #[test]
    fn cli_explicit_none_beats_env_timeout() {
        let env = PartialConfig {
            host: None,
            port: None,
            tls: None,
            timeout_ms: Some(Some(3000)),
        };
        let cli = PartialConfig {
            host: None,
            port: None,
            tls: None,
            timeout_ms: Some(None),
        };

        let merged = merge_config(defaults(), env, cli);
        assert_eq!(merged.timeout_ms, None);
    }
}
