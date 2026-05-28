#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub tls: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub tls: Option<bool>,
}

pub fn merge_config(
    defaults: &Config,
    profile: Option<&PartialConfig>,
    overrides: Option<&PartialConfig>,
) -> Config {
    let mut cfg = defaults.clone();

    if let Some(ovr) = overrides {
        if let Some(host) = &ovr.host {
            cfg.host = host.clone();
        }
        if let Some(port) = ovr.port {
            cfg.port = port;
        }
        if let Some(tls) = ovr.tls {
            cfg.tls = tls;
        }
    }

    if let Some(profile) = profile {
        if let Some(host) = &profile.host {
            cfg.host = host.clone();
        }
        if let Some(port) = profile.port {
            cfg.port = port;
        }
        if let Some(tls) = profile.tls {
            cfg.tls = tls;
        }
    }

    cfg
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "localhost".into(),
            port: 8080,
            tls: false,
        }
    }

    #[test]
    fn profile_overrides_defaults_when_no_explicit_override() {
        let profile = PartialConfig {
            host: Some("profile.internal".into()),
            port: None,
            tls: Some(true),
        };

        let merged = merge_config(&defaults(), Some(&profile), None);

        assert_eq!(
            merged,
            Config {
                host: "profile.internal".into(),
                port: 8080,
                tls: true,
            }
        );
    }

    #[test]
    fn explicit_overrides_win_over_profile_and_defaults() {
        let profile = PartialConfig {
            host: Some("profile.internal".into()),
            port: Some(9000),
            tls: Some(true),
        };
        let overrides = PartialConfig {
            host: None,
            port: Some(7000),
            tls: Some(false),
        };

        let merged = merge_config(&defaults(), Some(&profile), Some(&overrides));

        assert_eq!(merged.host, "profile.internal");
        assert_eq!(merged.port, 7000);
        assert!(!merged.tls);
    }

    #[test]
    fn override_only_replaces_specified_fields() {
        let profile = PartialConfig {
            host: Some("profile.internal".into()),
            port: Some(9000),
            tls: None,
        };
        let overrides = PartialConfig {
            host: Some("cli.example".into()),
            port: None,
            tls: None,
        };

        let merged = merge_config(&defaults(), Some(&profile), Some(&overrides));

        assert_eq!(
            merged,
            Config {
                host: "cli.example".into(),
                port: 9000,
                tls: false,
            }
        );
    }
}
