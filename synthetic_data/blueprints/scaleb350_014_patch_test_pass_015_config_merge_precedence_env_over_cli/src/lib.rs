#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub host: String,
    pub port: u16,
    pub tls: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialSettings {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub tls: Option<bool>,
}

pub fn merge_settings(
    defaults: PartialSettings,
    file: PartialSettings,
    env: PartialSettings,
    cli: PartialSettings,
) -> Settings {
    Settings {
        host: defaults
            .host
            .or(file.host)
            .or(cli.host)
            .or(env.host)
            .unwrap_or_else(|| "127.0.0.1".to_string()),
        port: defaults.port.or(file.port).or(cli.port).or(env.port).unwrap_or(8080),
        tls: defaults.tls.or(file.tls).or(cli.tls).or(env.tls).unwrap_or(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(host: Option<&str>, port: Option<u16>, tls: Option<bool>) -> PartialSettings {
        PartialSettings {
            host: host.map(str::to_string),
            port,
            tls,
        }
    }

    #[test]
    fn precedence_is_cli_then_env_then_file_then_defaults() {
        let merged = merge_settings(
            p(Some("default.local"), Some(80), Some(false)),
            p(Some("file.local"), Some(8080), Some(false)),
            p(Some("env.local"), Some(9090), Some(true)),
            p(Some("cli.local"), Some(3000), Some(false)),
        );

        assert_eq!(
            merged,
            Settings {
                host: "cli.local".to_string(),
                port: 3000,
                tls: false,
            }
        );
    }

    #[test]
    fn env_beats_file_when_cli_missing() {
        let merged = merge_settings(
            p(Some("default.local"), Some(80), Some(false)),
            p(Some("file.local"), Some(8080), Some(false)),
            p(Some("env.local"), Some(9090), Some(true)),
            p(None, None, None),
        );

        assert_eq!(
            merged,
            Settings {
                host: "env.local".to_string(),
                port: 9090,
                tls: true,
            }
        );
    }

    #[test]
    fn mixed_missing_values_fall_back_per_field() {
        let merged = merge_settings(
            p(Some("default.local"), Some(80), Some(false)),
            p(None, Some(8080), None),
            p(Some("env.local"), None, Some(true)),
            p(None, Some(3000), None),
        );

        assert_eq!(
            merged,
            Settings {
                host: "env.local".to_string(),
                port: 3000,
                tls: true,
            }
        );
    }

    #[test]
    fn hardcoded_fallbacks_apply_when_all_layers_missing() {
        let merged = merge_settings(
            p(None, None, None),
            p(None, None, None),
            p(None, None, None),
            p(None, None, None),
        );

        assert_eq!(
            merged,
            Settings {
                host: "127.0.0.1".to_string(),
                port: 8080,
                tls: false,
            }
        );
    }
}
