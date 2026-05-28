#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub tls: bool,
    pub port_locked: bool,
}

impl Config {
    pub fn defaults() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            tls: false,
            port_locked: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub tls: Option<bool>,
    pub port_locked: Option<bool>,
}

pub fn merge_config(defaults: Config, file: PartialConfig, cli: PartialConfig) -> Config {
    let mut cfg = defaults;

    if let Some(host) = file.host {
        cfg.host = host;
    }
    if let Some(port) = file.port {
        cfg.port = port;
    }
    if let Some(tls) = file.tls {
        cfg.tls = tls;
    }
    if let Some(locked) = file.port_locked {
        cfg.port_locked = locked;
    }

    if let Some(host) = cli.host {
        cfg.host = host;
    }
    if let Some(port) = cli.port {
        cfg.port = port;
    }
    if let Some(tls) = cli.tls {
        cfg.tls = tls;
    }
    if let Some(locked) = cli.port_locked {
        cfg.port_locked = locked;
    }

    cfg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_overrides_file_for_normal_fields() {
        let defaults = Config::defaults();
        let file = PartialConfig {
            host: Some("file.local".into()),
            port: Some(9000),
            tls: Some(true),
            port_locked: Some(false),
        };
        let cli = PartialConfig {
            host: Some("cli.local".into()),
            port: Some(7000),
            tls: Some(false),
            port_locked: None,
        };

        let merged = merge_config(defaults, file, cli);
        assert_eq!(merged.host, "cli.local");
        assert_eq!(merged.port, 7000);
        assert!(!merged.tls);
        assert!(!merged.port_locked);
    }

    #[test]
    fn empty_cli_host_does_not_replace_file_host() {
        let defaults = Config::defaults();
        let file = PartialConfig {
            host: Some("service.internal".into()),
            port: None,
            tls: None,
            port_locked: None,
        };
        let cli = PartialConfig {
            host: Some(String::new()),
            port: None,
            tls: None,
            port_locked: None,
        };

        let merged = merge_config(defaults, file, cli);
        assert_eq!(merged.host, "service.internal");
    }

    #[test]
    fn locked_port_from_file_blocks_cli_port_override() {
        let defaults = Config::defaults();
        let file = PartialConfig {
            host: None,
            port: Some(8443),
            tls: Some(true),
            port_locked: Some(true),
        };
        let cli = PartialConfig {
            host: None,
            port: Some(3000),
            tls: None,
            port_locked: None,
        };

        let merged = merge_config(defaults, file, cli);
        assert_eq!(merged.port, 8443);
        assert!(merged.port_locked);
        assert!(merged.tls);
    }

    #[test]
    fn cli_can_lock_after_setting_port() {
        let defaults = Config::defaults();
        let file = PartialConfig {
            host: None,
            port: Some(5000),
            tls: None,
            port_locked: Some(false),
        };
        let cli = PartialConfig {
            host: None,
            port: Some(6000),
            tls: None,
            port_locked: Some(true),
        };

        let merged = merge_config(defaults, file, cli);
        assert_eq!(merged.port, 6000);
        assert!(merged.port_locked);
    }
}
