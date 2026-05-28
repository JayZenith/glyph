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
    overrides: &PartialConfig,
) -> Config {
    Config {
        host: overrides
            .host
            .clone()
            .or_else(|| env.host.clone())
            .unwrap_or_else(|| defaults.host.clone()),
        port: env.port.or(overrides.port).unwrap_or(defaults.port),
        tls: defaults.tls,
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
        }
    }

    #[test]
    fn uses_defaults_when_no_layers_supply_values() {
        let env = PartialConfig::default();
        let overrides = PartialConfig::default();
        assert_eq!(merge_config(&defaults(), &env, &overrides), defaults());
    }

    #[test]
    fn env_overrides_defaults() {
        let env = PartialConfig {
            host: Some("env.internal".into()),
            port: Some(9000),
            tls: Some(true),
        };
        let overrides = PartialConfig::default();

        assert_eq!(
            merge_config(&defaults(), &env, &overrides),
            Config {
                host: "env.internal".into(),
                port: 9000,
                tls: true,
            }
        );
    }

    #[test]
    fn explicit_overrides_beat_env_and_defaults() {
        let env = PartialConfig {
            host: Some("env.internal".into()),
            port: Some(9000),
            tls: Some(false),
        };
        let overrides = PartialConfig {
            host: Some("cli.example".into()),
            port: Some(7000),
            tls: Some(true),
        };

        assert_eq!(
            merge_config(&defaults(), &env, &overrides),
            Config {
                host: "cli.example".into(),
                port: 7000,
                tls: true,
            }
        );
    }

    #[test]
    fn each_field_uses_highest_precedence_available_independently() {
        let env = PartialConfig {
            host: None,
            port: Some(9100),
            tls: Some(true),
        };
        let overrides = PartialConfig {
            host: Some("override.host".into()),
            port: None,
            tls: None,
        };

        assert_eq!(
            merge_config(&defaults(), &env, &overrides),
            Config {
                host: "override.host".into(),
                port: 9100,
                tls: true,
            }
        );
    }
}
