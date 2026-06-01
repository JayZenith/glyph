#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub use_tls: Option<bool>,
}

impl Config {
    pub fn new(host: Option<&str>, port: Option<u16>, use_tls: Option<bool>) -> Self {
        Self {
            host: host.map(str::to_string),
            port,
            use_tls,
        }
    }
}

pub fn merge_config(defaults: &Config, env: &Config, cli: &Config) -> Config {
    Config {
        host: defaults
            .host
            .clone()
            .or_else(|| env.host.clone())
            .or_else(|| cli.host.clone()),
        port: cli.port.or(env.port).or(defaults.port),
        use_tls: cli.use_tls.or(env.use_tls).or(defaults.use_tls),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precedence_is_cli_then_env_then_defaults() {
        let defaults = Config::new(Some("default.local"), Some(80), Some(false));
        let env = Config::new(Some("env.local"), Some(8080), Some(false));
        let cli = Config::new(Some("cli.local"), Some(3000), Some(true));

        let merged = merge_config(&defaults, &env, &cli);

        assert_eq!(
            merged,
            Config {
                host: Some("cli.local".into()),
                port: Some(3000),
                use_tls: Some(true),
            }
        );
    }

    #[test]
    fn empty_cli_host_is_an_explicit_override() {
        let defaults = Config::new(Some("default.local"), Some(80), Some(false));
        let env = Config::new(Some("env.local"), Some(8080), Some(true));
        let cli = Config::new(Some(""), None, None);

        let merged = merge_config(&defaults, &env, &cli);

        assert_eq!(merged.host, Some(String::new()));
        assert_eq!(merged.port, Some(8080));
        assert_eq!(merged.use_tls, Some(true));
    }

    #[test]
    fn falls_back_per_field_when_higher_layers_are_missing() {
        let defaults = Config::new(Some("default.local"), Some(80), Some(false));
        let env = Config::new(None, Some(8080), None);
        let cli = Config::new(None, None, Some(true));

        let merged = merge_config(&defaults, &env, &cli);

        assert_eq!(merged.host, Some("default.local".into()));
        assert_eq!(merged.port, Some(8080));
        assert_eq!(merged.use_tls, Some(true));
    }
}
