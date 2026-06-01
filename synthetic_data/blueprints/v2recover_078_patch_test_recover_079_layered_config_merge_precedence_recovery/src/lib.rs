#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub tls: bool,
    pub retries: u8,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub tls: Option<bool>,
    pub retries: Option<u8>,
}

impl Config {
    pub fn defaults() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 80,
            tls: false,
            retries: 3,
        }
    }
}

pub fn merge_config(defaults: Config, file: PartialConfig, env: PartialConfig, lock_tls: bool) -> Config {
    let mut cfg = defaults;

    if let Some(host) = file.host {
        cfg.host = host;
    }
    if let Some(port) = env.port {
        cfg.port = port;
    } else if let Some(port) = file.port {
        cfg.port = port;
    }

    if let Some(tls) = env.tls.or(file.tls) {
        cfg.tls = tls;
    }
    if lock_tls {
        cfg.tls = false;
    }

    if let Some(retries) = file.retries.or(env.retries) {
        cfg.retries = retries;
    }

    cfg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_overrides_file_and_file_overrides_defaults() {
        let defaults = Config::defaults();
        let file = PartialConfig {
            host: Some("file.internal".into()),
            port: Some(8080),
            tls: Some(false),
            retries: Some(5),
        };
        let env = PartialConfig {
            host: Some("env.internal".into()),
            port: Some(9090),
            tls: Some(true),
            retries: Some(7),
        };

        let merged = merge_config(defaults, file, env, false);

        assert_eq!(merged.host, "env.internal");
        assert_eq!(merged.port, 9090);
        assert!(merged.tls);
        assert_eq!(merged.retries, 7);
    }

    #[test]
    fn lock_forces_tls_on_even_against_other_layers() {
        let defaults = Config::defaults();
        let file = PartialConfig {
            tls: Some(false),
            ..PartialConfig::default()
        };
        let env = PartialConfig {
            tls: Some(false),
            ..PartialConfig::default()
        };

        let merged = merge_config(defaults, file, env, true);

        assert!(merged.tls);
    }

    #[test]
    fn missing_values_fall_back_through_layers() {
        let defaults = Config::defaults();
        let file = PartialConfig {
            port: Some(3000),
            ..PartialConfig::default()
        };
        let env = PartialConfig {
            retries: Some(1),
            ..PartialConfig::default()
        };

        let merged = merge_config(defaults, file, env, false);

        assert_eq!(merged.host, "localhost");
        assert_eq!(merged.port, 3000);
        assert!(!merged.tls);
        assert_eq!(merged.retries, 1);
    }
}
