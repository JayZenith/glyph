#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub debug: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub debug: Option<bool>,
}

pub fn merge_config(
    defaults: &Config,
    base: &PartialConfig,
    env: &PartialConfig,
) -> Config {
    Config {
        host: base
            .host
            .clone()
            .or_else(|| env.host.clone())
            .unwrap_or_else(|| defaults.host.clone()),
        port: base.port.or(env.port).unwrap_or(defaults.port),
        debug: base.debug.or(env.debug).unwrap_or(defaults.debug),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "localhost".into(),
            port: 8080,
            debug: false,
        }
    }

    #[test]
    fn base_overrides_defaults_when_env_missing() {
        let base = PartialConfig {
            host: Some("service.internal".into()),
            port: Some(9000),
            debug: Some(true),
        };
        let env = PartialConfig::default();

        let merged = merge_config(&defaults(), &base, &env);
        assert_eq!(
            merged,
            Config {
                host: "service.internal".into(),
                port: 9000,
                debug: true,
            }
        );
    }

    #[test]
    fn env_overrides_base_only_for_fields_it_sets() {
        let base = PartialConfig {
            host: Some("service.internal".into()),
            port: Some(9000),
            debug: Some(false),
        };
        let env = PartialConfig {
            host: None,
            port: Some(7000),
            debug: Some(true),
        };

        let merged = merge_config(&defaults(), &base, &env);
        assert_eq!(
            merged,
            Config {
                host: "service.internal".into(),
                port: 7000,
                debug: true,
            }
        );
    }

    #[test]
    fn defaults_fill_unset_fields_after_other_layers() {
        let base = PartialConfig {
            host: None,
            port: Some(5000),
            debug: None,
        };
        let env = PartialConfig {
            host: Some("env.example".into()),
            port: None,
            debug: None,
        };

        let merged = merge_config(&defaults(), &base, &env);
        assert_eq!(
            merged,
            Config {
                host: "env.example".into(),
                port: 5000,
                debug: false,
            }
        );
    }
}
