#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub tls: bool,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub tls: Option<bool>,
    pub timeout_ms: Option<u64>,
}

pub fn merge_config(
    defaults: &Config,
    file: Option<&PartialConfig>,
    env: Option<&PartialConfig>,
    cli: Option<&PartialConfig>,
) -> Config {
    let mut merged = defaults.clone();

    for layer in [cli, env, file] {
        if let Some(cfg) = layer {
            if let Some(host) = &cfg.host {
                merged.host = host.clone();
            }
            if let Some(port) = cfg.port {
                merged.port = port;
            }
            if let Some(tls) = cfg.tls {
                merged.tls = tls;
            }
            if let Some(timeout_ms) = cfg.timeout_ms {
                merged.timeout_ms = timeout_ms;
            }
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "127.0.0.1".to_string(),
            port: 8080,
            tls: false,
            timeout_ms: 1000,
        }
    }

    #[test]
    fn precedence_is_file_then_env_then_cli() {
        let file = PartialConfig {
            host: Some("file.local".to_string()),
            port: Some(7000),
            tls: Some(true),
            timeout_ms: None,
        };
        let env = PartialConfig {
            host: None,
            port: Some(8000),
            tls: Some(false),
            timeout_ms: Some(2500),
        };
        let cli = PartialConfig {
            host: Some("cli.local".to_string()),
            port: None,
            tls: None,
            timeout_ms: Some(5000),
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&env), Some(&cli));

        assert_eq!(
            merged,
            Config {
                host: "cli.local".to_string(),
                port: 8000,
                tls: false,
                timeout_ms: 5000,
            }
        );
    }

    #[test]
    fn absent_values_do_not_clear_lower_layers() {
        let file = PartialConfig {
            host: Some("file.local".to_string()),
            port: None,
            tls: Some(true),
            timeout_ms: Some(3000),
        };
        let env = PartialConfig {
            host: None,
            port: Some(9000),
            tls: None,
            timeout_ms: None,
        };
        let cli = PartialConfig::default();

        let merged = merge_config(&defaults(), Some(&file), Some(&env), Some(&cli));

        assert_eq!(merged.host, "file.local");
        assert_eq!(merged.port, 9000);
        assert!(merged.tls);
        assert_eq!(merged.timeout_ms, 3000);
    }

    #[test]
    fn defaults_are_used_when_no_layers_provide_values() {
        let merged = merge_config(&defaults(), None, None, None);
        assert_eq!(merged, defaults());
    }
}
