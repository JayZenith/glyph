#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub use_tls: bool,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub use_tls: Option<bool>,
    pub timeout_ms: Option<u64>,
}

pub fn merge_config(
    defaults: &Config,
    file: Option<&PartialConfig>,
    env: Option<&PartialConfig>,
    cli: Option<&PartialConfig>,
) -> Config {
    let mut out = defaults.clone();

    for layer in [file, env, cli] {
        if let Some(layer) = layer {
            if let Some(host) = &layer.host {
                out.host = host.clone();
            }
            if let Some(port) = layer.port {
                out.port = port;
            }
            if let Some(use_tls) = layer.use_tls {
                out.use_tls = use_tls;
            }
            if let Some(timeout_ms) = layer.timeout_ms {
                out.timeout_ms = timeout_ms;
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Config {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            use_tls: false,
            timeout_ms: 1000,
        }
    }

    #[test]
    fn keeps_defaults_when_no_overrides_exist() {
        let merged = merge_config(&base(), None, None, None);
        assert_eq!(merged, base());
    }

    #[test]
    fn later_layers_override_earlier_layers() {
        let file = PartialConfig {
            host: Some("file.internal".to_string()),
            port: Some(9000),
            use_tls: Some(false),
            timeout_ms: None,
        };
        let env = PartialConfig {
            host: None,
            port: Some(7000),
            use_tls: Some(true),
            timeout_ms: Some(2500),
        };
        let cli = PartialConfig {
            host: Some("cli.example.com".to_string()),
            port: None,
            use_tls: None,
            timeout_ms: Some(5000),
        };

        let merged = merge_config(&base(), Some(&file), Some(&env), Some(&cli));

        assert_eq!(
            merged,
            Config {
                host: "cli.example.com".to_string(),
                port: 7000,
                use_tls: true,
                timeout_ms: 5000,
            }
        );
    }

    #[test]
    fn missing_values_do_not_clear_earlier_values() {
        let file = PartialConfig {
            host: Some("from-file".to_string()),
            port: None,
            use_tls: Some(true),
            timeout_ms: Some(3000),
        };
        let env = PartialConfig {
            host: None,
            port: Some(8100),
            use_tls: None,
            timeout_ms: None,
        };

        let merged = merge_config(&base(), Some(&file), Some(&env), None);

        assert_eq!(merged.host, "from-file");
        assert_eq!(merged.port, 8100);
        assert_eq!(merged.use_tls, true);
        assert_eq!(merged.timeout_ms, 3000);
    }
}
