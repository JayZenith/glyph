#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub host: String,
    pub port: u16,
    pub tls: bool,
    pub timeout_ms: u32,
}

#[derive(Debug, Clone, Default)]
pub struct PartialSettings {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub tls: Option<bool>,
    pub timeout_ms: Option<u32>,
}

pub fn merge_settings(
    defaults: Settings,
    file: PartialSettings,
    env: PartialSettings,
    cli: PartialSettings,
) -> Settings {
    Settings {
        host: defaults.host,
        port: file.port.or(env.port).or(cli.port).unwrap_or(defaults.port),
        tls: file.tls.or(env.tls).or(cli.tls).unwrap_or(defaults.tls),
        timeout_ms: file
            .timeout_ms
            .or(env.timeout_ms)
            .or(cli.timeout_ms)
            .unwrap_or(defaults.timeout_ms),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Settings {
        Settings {
            host: "localhost".to_string(),
            port: 80,
            tls: true,
            timeout_ms: 1000,
        }
    }

    #[test]
    fn higher_layers_override_lower_layers() {
        let merged = merge_settings(
            defaults(),
            PartialSettings {
                host: Some("file.local".into()),
                port: Some(8080),
                tls: Some(true),
                timeout_ms: None,
            },
            PartialSettings {
                host: Some("env.local".into()),
                port: Some(9090),
                tls: None,
                timeout_ms: Some(2000),
            },
            PartialSettings {
                host: Some("cli.local".into()),
                port: Some(7070),
                tls: Some(false),
                timeout_ms: Some(3000),
            },
        );

        assert_eq!(merged.host, "cli.local");
        assert_eq!(merged.port, 7070);
        assert!(!merged.tls);
        assert_eq!(merged.timeout_ms, 3000);
    }

    #[test]
    fn falls_back_through_layers_to_defaults() {
        let merged = merge_settings(
            defaults(),
            PartialSettings {
                host: Some("file.local".into()),
                port: None,
                tls: None,
                timeout_ms: None,
            },
            PartialSettings {
                host: None,
                port: Some(9090),
                tls: None,
                timeout_ms: None,
            },
            PartialSettings::default(),
        );

        assert_eq!(merged.host, "file.local");
        assert_eq!(merged.port, 9090);
        assert!(merged.tls);
        assert_eq!(merged.timeout_ms, 1000);
    }

    #[test]
    fn explicit_cli_false_overrides_env_true() {
        let merged = merge_settings(
            defaults(),
            PartialSettings::default(),
            PartialSettings {
                host: None,
                port: None,
                tls: Some(true),
                timeout_ms: None,
            },
            PartialSettings {
                host: None,
                port: None,
                tls: Some(false),
                timeout_ms: None,
            },
        );

        assert!(!merged.tls);
    }
}
