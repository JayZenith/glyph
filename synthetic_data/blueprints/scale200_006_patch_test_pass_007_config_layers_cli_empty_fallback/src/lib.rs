#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub profile: String,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub profile: Option<String>,
}

pub fn merge_config(
    defaults: &Config,
    file_cfg: &PartialConfig,
    env_cfg: &PartialConfig,
    cli_cfg: &PartialConfig,
) -> Config {
    Config {
        host: cli_cfg
            .host
            .clone()
            .or_else(|| env_cfg.host.clone())
            .or_else(|| file_cfg.host.clone())
            .unwrap_or_else(|| defaults.host.clone()),
        port: cli_cfg
            .port
            .or(env_cfg.port)
            .or(file_cfg.port)
            .unwrap_or(defaults.port),
        profile: cli_cfg
            .profile
            .clone()
            .or_else(|| env_cfg.profile.clone())
            .or_else(|| file_cfg.profile.clone())
            .unwrap_or_else(|| defaults.profile.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "127.0.0.1".into(),
            port: 8080,
            profile: "dev".into(),
        }
    }

    #[test]
    fn precedence_is_default_then_file_then_env_then_cli() {
        let merged = merge_config(
            &defaults(),
            &PartialConfig {
                host: Some("file.local".into()),
                port: Some(7000),
                profile: Some("file".into()),
            },
            &PartialConfig {
                host: Some("env.local".into()),
                port: Some(8000),
                profile: Some("env".into()),
            },
            &PartialConfig {
                host: Some("cli.local".into()),
                port: Some(9000),
                profile: Some("cli".into()),
            },
        );

        assert_eq!(
            merged,
            Config {
                host: "cli.local".into(),
                port: 9000,
                profile: "cli".into(),
            }
        );
    }

    #[test]
    fn empty_cli_strings_do_not_override_lower_layers() {
        let merged = merge_config(
            &defaults(),
            &PartialConfig {
                host: Some("file.local".into()),
                port: Some(7000),
                profile: Some("file".into()),
            },
            &PartialConfig {
                host: Some("env.local".into()),
                port: None,
                profile: Some("env".into()),
            },
            &PartialConfig {
                host: Some("".into()),
                port: Some(9100),
                profile: Some("".into()),
            },
        );

        assert_eq!(merged.host, "env.local");
        assert_eq!(merged.port, 9100);
        assert_eq!(merged.profile, "env");
    }

    #[test]
    fn empty_cli_uses_file_then_default_when_needed() {
        let merged = merge_config(
            &defaults(),
            &PartialConfig {
                host: None,
                port: None,
                profile: Some("from-file".into()),
            },
            &PartialConfig::default(),
            &PartialConfig {
                host: Some("".into()),
                port: None,
                profile: Some("".into()),
            },
        );

        assert_eq!(merged.host, "127.0.0.1");
        assert_eq!(merged.port, 8080);
        assert_eq!(merged.profile, "from-file");
    }
}
