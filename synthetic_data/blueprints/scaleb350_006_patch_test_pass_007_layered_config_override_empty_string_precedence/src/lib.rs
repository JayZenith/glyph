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

pub fn merge_config(defaults: &Config, file: &PartialConfig, env: &PartialConfig) -> Config {
    Config {
        host: env
            .host
            .clone()
            .or_else(|| file.host.clone())
            .unwrap_or_else(|| defaults.host.clone()),
        port: env.port.or(file.port).unwrap_or(defaults.port),
        debug: env.debug.or(file.debug).unwrap_or(defaults.debug),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Config {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            debug: false,
        }
    }

    #[test]
    fn env_overrides_file_and_defaults() {
        let file = PartialConfig {
            host: Some("file-host".to_string()),
            port: Some(3000),
            debug: Some(false),
        };
        let env = PartialConfig {
            host: Some("env-host".to_string()),
            port: Some(9000),
            debug: Some(true),
        };

        let merged = merge_config(&base(), &file, &env);
        assert_eq!(
            merged,
            Config {
                host: "env-host".to_string(),
                port: 9000,
                debug: true,
            }
        );
    }

    #[test]
    fn file_used_when_env_missing() {
        let file = PartialConfig {
            host: Some("file-host".to_string()),
            port: Some(3000),
            debug: Some(true),
        };
        let env = PartialConfig::default();

        let merged = merge_config(&base(), &file, &env);
        assert_eq!(
            merged,
            Config {
                host: "file-host".to_string(),
                port: 3000,
                debug: true,
            }
        );
    }

    #[test]
    fn defaults_used_when_no_overrides() {
        let merged = merge_config(&base(), &PartialConfig::default(), &PartialConfig::default());
        assert_eq!(merged, base());
    }

    #[test]
    fn empty_env_host_does_not_clobber_file_host() {
        let file = PartialConfig {
            host: Some("file-host".to_string()),
            port: None,
            debug: None,
        };
        let env = PartialConfig {
            host: Some(String::new()),
            port: None,
            debug: None,
        };

        let merged = merge_config(&base(), &file, &env);
        assert_eq!(merged.host, "file-host");
    }

    #[test]
    fn empty_env_host_falls_back_to_default_when_file_missing() {
        let env = PartialConfig {
            host: Some(String::new()),
            port: None,
            debug: None,
        };

        let merged = merge_config(&base(), &PartialConfig::default(), &env);
        assert_eq!(merged.host, "localhost");
    }
}
