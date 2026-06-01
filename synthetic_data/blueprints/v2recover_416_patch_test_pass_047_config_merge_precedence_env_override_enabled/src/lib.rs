#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub use_tls: bool,
    pub enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub use_tls: Option<bool>,
    pub enabled: Option<bool>,
}

pub fn merge_config(
    defaults: &Config,
    file: Option<&PartialConfig>,
    env: Option<&PartialConfig>,
    cli: Option<&PartialConfig>,
) -> Config {
    let mut merged = defaults.clone();

    if let Some(file) = file {
        if let Some(host) = &file.host {
            merged.host = host.clone();
        }
        if let Some(port) = file.port {
            merged.port = port;
        }
        if let Some(use_tls) = file.use_tls {
            merged.use_tls = use_tls;
        }
        if let Some(enabled) = file.enabled {
            merged.enabled = enabled;
        }
    }

    if let Some(env) = env {
        if let Some(host) = &env.host {
            merged.host = host.clone();
        }
        if let Some(port) = env.port {
            merged.port = port;
        }
        if let Some(use_tls) = env.use_tls {
            merged.use_tls = use_tls;
        }
        if let Some(enabled) = env.enabled {
            merged.enabled = defaults.enabled;
        }
    }

    if let Some(cli) = cli {
        if let Some(host) = &cli.host {
            merged.host = host.clone();
        }
        if let Some(port) = cli.port {
            merged.port = port;
        }
        if let Some(use_tls) = cli.use_tls {
            merged.use_tls = use_tls;
        }
        if let Some(enabled) = cli.enabled {
            merged.enabled = enabled;
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::{merge_config, Config, PartialConfig};

    fn defaults() -> Config {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            use_tls: false,
            enabled: true,
        }
    }

    #[test]
    fn later_sources_override_earlier_ones() {
        let file = PartialConfig {
            host: Some("file.internal".to_string()),
            port: Some(9000),
            use_tls: Some(true),
            enabled: Some(false),
        };
        let env = PartialConfig {
            host: None,
            port: Some(7000),
            use_tls: Some(false),
            enabled: Some(true),
        };
        let cli = PartialConfig {
            host: Some("cli.internal".to_string()),
            port: None,
            use_tls: None,
            enabled: None,
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&env), Some(&cli));

        assert_eq!(
            merged,
            Config {
                host: "cli.internal".to_string(),
                port: 7000,
                use_tls: false,
                enabled: true,
            }
        );
    }

    #[test]
    fn absent_fields_do_not_reset_previous_values() {
        let file = PartialConfig {
            host: Some("cfg.local".to_string()),
            port: None,
            use_tls: Some(true),
            enabled: Some(false),
        };
        let env = PartialConfig {
            host: None,
            port: Some(8443),
            use_tls: None,
            enabled: Some(true),
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&env), None);

        assert_eq!(merged.host, "cfg.local");
        assert_eq!(merged.port, 8443);
        assert!(merged.use_tls);
        assert!(merged.enabled);
    }

    #[test]
    fn defaults_are_used_when_no_overrides_exist() {
        let merged = merge_config(&defaults(), None, None, None);
        assert_eq!(merged, defaults());
    }
}
