#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub use_tls: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub use_tls: Option<bool>,
}

pub fn merge_config(
    defaults: &Config,
    env: Option<&PartialConfig>,
    cli: Option<&PartialConfig>,
) -> Config {
    let env = env.cloned().unwrap_or_default();
    let cli = cli.cloned().unwrap_or_default();

    Config {
        host: cli.host.or(env.host).unwrap_or_else(|| defaults.host.clone()),
        port: cli.port.or(env.port).unwrap_or(defaults.port),
        use_tls: cli.use_tls.or(env.use_tls).unwrap_or(defaults.use_tls),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            use_tls: false,
        }
    }

    #[test]
    fn cli_overrides_env_and_defaults() {
        let env = PartialConfig {
            host: Some("env.local".to_string()),
            port: Some(9000),
            use_tls: Some(false),
        };
        let cli = PartialConfig {
            host: Some("cli.local".to_string()),
            port: Some(7000),
            use_tls: Some(true),
        };

        let merged = merge_config(&defaults(), Some(&env), Some(&cli));
        assert_eq!(
            merged,
            Config {
                host: "cli.local".to_string(),
                port: 7000,
                use_tls: true,
            }
        );
    }

    #[test]
    fn env_fills_missing_values_when_cli_absent() {
        let env = PartialConfig {
            host: None,
            port: Some(9000),
            use_tls: Some(true),
        };

        let merged = merge_config(&defaults(), Some(&env), None);
        assert_eq!(
            merged,
            Config {
                host: "localhost".to_string(),
                port: 9000,
                use_tls: true,
            }
        );
    }

    #[test]
    fn false_cli_flag_still_overrides_true_env_flag() {
        let env = PartialConfig {
            host: None,
            port: None,
            use_tls: Some(true),
        };
        let cli = PartialConfig {
            host: None,
            port: None,
            use_tls: Some(false),
        };

        let merged = merge_config(&defaults(), Some(&env), Some(&cli));
        assert!(!merged.use_tls);
    }

    #[test]
    fn defaults_used_when_no_overrides_present() {
        let merged = merge_config(&defaults(), None, None);
        assert_eq!(merged, defaults());
    }
}
