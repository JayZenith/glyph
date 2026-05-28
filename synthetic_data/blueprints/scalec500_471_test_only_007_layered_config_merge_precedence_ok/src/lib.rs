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

pub fn merge_config(
    defaults: &Config,
    file_cfg: &PartialConfig,
    override_cfg: &PartialConfig,
) -> Config {
    Config {
        host: override_cfg
            .host
            .clone()
            .or_else(|| file_cfg.host.clone())
            .unwrap_or_else(|| defaults.host.clone()),
        port: override_cfg.port.or(file_cfg.port).unwrap_or(defaults.port),
        use_tls: override_cfg
            .use_tls
            .or(file_cfg.use_tls)
            .unwrap_or(defaults.use_tls),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            use_tls: false,
        }
    }

    #[test]
    fn uses_defaults_when_no_layers_provide_values() {
        let file_cfg = PartialConfig::default();
        let override_cfg = PartialConfig::default();

        let merged = merge_config(&defaults(), &file_cfg, &override_cfg);

        assert_eq!(
            merged,
            Config {
                host: "localhost".to_string(),
                port: 8080,
                use_tls: false,
            }
        );
    }

    #[test]
    fn file_config_overrides_defaults() {
        let file_cfg = PartialConfig {
            host: Some("file.example".to_string()),
            port: Some(9000),
            use_tls: Some(true),
        };
        let override_cfg = PartialConfig::default();

        let merged = merge_config(&defaults(), &file_cfg, &override_cfg);

        assert_eq!(merged.host, "file.example");
        assert_eq!(merged.port, 9000);
        assert!(merged.use_tls);
    }

    #[test]
    fn explicit_overrides_win_over_file_config() {
        let file_cfg = PartialConfig {
            host: Some("file.example".to_string()),
            port: Some(9000),
            use_tls: Some(false),
        };
        let override_cfg = PartialConfig {
            host: Some("cli.example".to_string()),
            port: Some(7000),
            use_tls: Some(true),
        };

        let merged = merge_config(&defaults(), &file_cfg, &override_cfg);

        assert_eq!(merged.host, "cli.example");
        assert_eq!(merged.port, 7000);
        assert!(merged.use_tls);
    }

    #[test]
    fn merge_can_mix_sources_per_field() {
        let file_cfg = PartialConfig {
            host: Some("file.example".to_string()),
            port: None,
            use_tls: Some(true),
        };
        let override_cfg = PartialConfig {
            host: None,
            port: Some(7000),
            use_tls: None,
        };

        let merged = merge_config(&defaults(), &file_cfg, &override_cfg);

        assert_eq!(
            merged,
            Config {
                host: "file.example".to_string(),
                port: 7000,
                use_tls: true,
            }
        );
    }
}
