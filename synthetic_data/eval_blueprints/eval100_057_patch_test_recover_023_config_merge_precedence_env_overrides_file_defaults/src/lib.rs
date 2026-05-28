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
    file_cfg: &PartialConfig,
    env_cfg: &PartialConfig,
) -> Config {
    Config {
        host: file_cfg
            .host
            .clone()
            .or_else(|| env_cfg.host.clone())
            .unwrap_or_else(|| defaults.host.clone()),
        port: env_cfg.port.or(file_cfg.port).unwrap_or(defaults.port),
        debug: file_cfg.debug.or(env_cfg.debug).unwrap_or(defaults.debug),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            debug: false,
        }
    }

    #[test]
    fn env_overrides_file_for_all_fields() {
        let file_cfg = PartialConfig {
            host: Some("file.internal".to_string()),
            port: Some(3000),
            debug: Some(false),
        };
        let env_cfg = PartialConfig {
            host: Some("env.internal".to_string()),
            port: Some(9000),
            debug: Some(true),
        };

        let merged = merge_config(&defaults(), &file_cfg, &env_cfg);
        assert_eq!(
            merged,
            Config {
                host: "env.internal".to_string(),
                port: 9000,
                debug: true,
            }
        );
    }

    #[test]
    fn empty_env_host_does_not_override_file_or_default() {
        let file_cfg = PartialConfig {
            host: Some("file.internal".to_string()),
            port: None,
            debug: None,
        };
        let env_cfg = PartialConfig {
            host: Some(String::new()),
            port: None,
            debug: None,
        };

        let merged = merge_config(&defaults(), &file_cfg, &env_cfg);
        assert_eq!(merged.host, "file.internal");

        let merged2 = merge_config(
            &defaults(),
            &PartialConfig::default(),
            &env_cfg,
        );
        assert_eq!(merged2.host, "localhost");
    }

    #[test]
    fn missing_values_fall_back_in_order() {
        let file_cfg = PartialConfig {
            host: None,
            port: Some(7000),
            debug: None,
        };
        let env_cfg = PartialConfig {
            host: None,
            port: None,
            debug: Some(true),
        };

        let merged = merge_config(&defaults(), &file_cfg, &env_cfg);
        assert_eq!(
            merged,
            Config {
                host: "localhost".to_string(),
                port: 7000,
                debug: true,
            }
        );
    }
}
