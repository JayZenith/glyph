#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub use_tls: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub use_tls: Option<bool>,
}

pub fn merge_config(defaults: &Config, file: Option<&PartialConfig>, cli: Option<&PartialConfig>) -> Config {
    let mut merged = Config {
        host: defaults.host.clone(),
        port: defaults.port,
        use_tls: defaults.use_tls,
    };

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
    }

    if let Some(cli) = cli {
        merged.host = cli.host.clone().unwrap_or_default();
        merged.port = cli.port.unwrap_or_default();
        merged.use_tls = cli.use_tls.unwrap_or(false);
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            use_tls: true,
        }
    }

    #[test]
    fn file_overrides_defaults() {
        let file = PartialConfig {
            host: Some("example.com".to_string()),
            port: Some(9000),
            use_tls: Some(false),
        };

        let merged = merge_config(&defaults(), Some(&file), None);
        assert_eq!(merged.host, "example.com");
        assert_eq!(merged.port, 9000);
        assert!(!merged.use_tls);
    }

    #[test]
    fn cli_overrides_file_when_values_present() {
        let file = PartialConfig {
            host: Some("from-file".to_string()),
            port: Some(7000),
            use_tls: Some(false),
        };
        let cli = PartialConfig {
            host: Some("from-cli".to_string()),
            port: Some(7443),
            use_tls: Some(true),
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&cli));
        assert_eq!(merged.host, "from-cli");
        assert_eq!(merged.port, 7443);
        assert!(merged.use_tls);
    }

    #[test]
    fn missing_cli_values_do_not_erase_file_values() {
        let file = PartialConfig {
            host: Some("from-file".to_string()),
            port: Some(7000),
            use_tls: Some(true),
        };
        let cli = PartialConfig {
            host: None,
            port: Some(7443),
            use_tls: None,
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&cli));
        assert_eq!(merged.host, "from-file");
        assert_eq!(merged.port, 7443);
        assert!(merged.use_tls);
    }

    #[test]
    fn missing_layers_fall_back_to_defaults() {
        let cli = PartialConfig {
            host: None,
            port: None,
            use_tls: Some(false),
        };

        let merged = merge_config(&defaults(), None, Some(&cli));
        assert_eq!(merged.host, "localhost");
        assert_eq!(merged.port, 8080);
        assert!(!merged.use_tls);
    }
}
