#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub debug: Option<bool>,
}

impl Config {
    pub fn new(host: Option<&str>, port: Option<u16>, debug: Option<bool>) -> Self {
        Self {
            host: host.map(str::to_string),
            port,
            debug,
        }
    }
}

pub fn merge_configs(defaults: &Config, env: &Config, cli: &Config) -> Config {
    let mut merged = defaults.clone();

    merged.host = env.host.clone();
    merged.port = env.port.or(merged.port);
    merged.debug = env.debug.or(merged.debug);

    merged.host = cli.host.clone().or(merged.host);
    merged.port = cli.port.or(merged.port);
    merged.debug = cli.debug.or(merged.debug);

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_overrides_env_and_defaults() {
        let defaults = Config::new(Some("localhost"), Some(8080), Some(false));
        let env = Config::new(Some("env-host"), Some(9000), Some(false));
        let cli = Config::new(Some("cli-host"), Some(7000), Some(true));

        let merged = merge_configs(&defaults, &env, &cli);

        assert_eq!(
            merged,
            Config {
                host: Some("cli-host".to_string()),
                port: Some(7000),
                debug: Some(true),
            }
        );
    }

    #[test]
    fn env_fills_missing_cli_values_without_erasing_defaults() {
        let defaults = Config::new(Some("localhost"), Some(8080), Some(false));
        let env = Config::new(Some("env-host"), None, Some(true));
        let cli = Config::new(None, Some(7000), None);

        let merged = merge_configs(&defaults, &env, &cli);

        assert_eq!(
            merged,
            Config {
                host: Some("env-host".to_string()),
                port: Some(7000),
                debug: Some(true),
            }
        );
    }

    #[test]
    fn missing_higher_precedence_host_keeps_lower_precedence_value() {
        let defaults = Config::new(Some("localhost"), Some(8080), Some(false));
        let env = Config::new(None, Some(9000), None);
        let cli = Config::new(None, None, Some(true));

        let merged = merge_configs(&defaults, &env, &cli);

        assert_eq!(
            merged,
            Config {
                host: Some("localhost".to_string()),
                port: Some(9000),
                debug: Some(true),
            }
        );
    }
}
