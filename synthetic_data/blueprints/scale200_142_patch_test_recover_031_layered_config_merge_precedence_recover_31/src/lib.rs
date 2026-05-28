#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub use_tls: Option<bool>,
    pub timeout_ms: Option<u64>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            host: None,
            port: None,
            use_tls: None,
            timeout_ms: None,
        }
    }
}

pub fn merge_config(defaults: &Config, env: &Config, cli: &Config) -> Config {
    Config {
        host: defaults.host.clone().or_else(|| env.host.clone()).or_else(|| cli.host.clone()),
        port: defaults.port.or(env.port).or(cli.port),
        use_tls: defaults.use_tls.or(env.use_tls).or(cli.use_tls),
        timeout_ms: defaults.timeout_ms.or(env.timeout_ms).or(cli.timeout_ms),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(host: Option<&str>, port: Option<u16>, use_tls: Option<bool>, timeout_ms: Option<u64>) -> Config {
        Config {
            host: host.map(str::to_string),
            port,
            use_tls,
            timeout_ms,
        }
    }

    #[test]
    fn cli_overrides_env_and_defaults_for_strings_and_numbers() {
        let defaults = cfg(Some("default.local"), Some(80), Some(false), Some(1000));
        let env = cfg(Some("env.local"), Some(8080), None, Some(2000));
        let cli = cfg(Some("cli.local"), Some(9090), None, None);

        let merged = merge_config(&defaults, &env, &cli);

        assert_eq!(merged.host.as_deref(), Some("cli.local"));
        assert_eq!(merged.port, Some(9090));
        assert_eq!(merged.use_tls, Some(false));
        assert_eq!(merged.timeout_ms, Some(2000));
    }

    #[test]
    fn explicit_false_from_higher_precedence_is_preserved() {
        let defaults = cfg(None, None, Some(true), None);
        let env = cfg(None, None, Some(false), None);
        let cli = cfg(None, None, None, None);

        let merged = merge_config(&defaults, &env, &cli);

        assert_eq!(merged.use_tls, Some(false));
    }

    #[test]
    fn lower_layers_fill_missing_values_only() {
        let defaults = cfg(Some("default.local"), Some(80), Some(true), Some(1000));
        let env = cfg(None, Some(8080), None, None);
        let cli = cfg(None, None, Some(false), None);

        let merged = merge_config(&defaults, &env, &cli);

        assert_eq!(merged.host.as_deref(), Some("default.local"));
        assert_eq!(merged.port, Some(8080));
        assert_eq!(merged.use_tls, Some(false));
        assert_eq!(merged.timeout_ms, Some(1000));
    }
}
