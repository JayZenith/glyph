#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub use_tls: bool,
    pub retries: u8,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub use_tls: Option<bool>,
    pub retries: Option<u8>,
}

pub fn merge_config(
    defaults: &Config,
    file: Option<&PartialConfig>,
    env: Option<&PartialConfig>,
    cli: Option<&PartialConfig>,
) -> Config {
    let mut merged = defaults.clone();

    if let Some(layer) = file {
        apply_layer(&mut merged, layer);
    }
    if let Some(layer) = env {
        apply_layer(&mut merged, layer);
    }
    if let Some(layer) = cli {
        apply_layer(&mut merged, layer);
    }

    merged
}

fn apply_layer(target: &mut Config, layer: &PartialConfig) {
    if let Some(host) = &layer.host {
        target.host = host.clone();
    }
    if let Some(port) = layer.port {
        target.port = port;
    }
    if let Some(use_tls) = layer.use_tls {
        if use_tls {
            target.use_tls = true;
        }
    }
    if let Some(retries) = layer.retries {
        if retries > 0 {
            target.retries = retries;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "localhost".into(),
            port: 8080,
            use_tls: true,
            retries: 3,
        }
    }

    #[test]
    fn later_layers_override_earlier_ones() {
        let file = PartialConfig {
            host: Some("file.internal".into()),
            port: Some(9000),
            use_tls: Some(true),
            retries: Some(5),
        };
        let env = PartialConfig {
            host: None,
            port: Some(7000),
            use_tls: Some(false),
            retries: None,
        };
        let cli = PartialConfig {
            host: Some("cli.example.com".into()),
            port: None,
            use_tls: None,
            retries: Some(0),
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&env), Some(&cli));

        assert_eq!(merged.host, "cli.example.com");
        assert_eq!(merged.port, 7000);
        assert!(!merged.use_tls);
        assert_eq!(merged.retries, 0);
    }

    #[test]
    fn none_values_do_not_override_existing_settings() {
        let file = PartialConfig {
            host: Some("from-file".into()),
            port: None,
            use_tls: None,
            retries: Some(4),
        };
        let env = PartialConfig::default();
        let cli = PartialConfig {
            host: None,
            port: Some(9090),
            use_tls: None,
            retries: None,
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&env), Some(&cli));

        assert_eq!(merged.host, "from-file");
        assert_eq!(merged.port, 9090);
        assert!(merged.use_tls);
        assert_eq!(merged.retries, 4);
    }
}
