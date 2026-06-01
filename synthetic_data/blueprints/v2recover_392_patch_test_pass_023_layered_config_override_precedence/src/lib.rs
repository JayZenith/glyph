#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub tls: bool,
    pub retries: u8,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub tls: Option<bool>,
    pub retries: Option<u8>,
}

impl PartialConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn merge_config(
    defaults: &Config,
    base: Option<&PartialConfig>,
    override_cfg: Option<&PartialConfig>,
) -> Config {
    let mut merged = defaults.clone();

    if let Some(ovr) = override_cfg {
        if let Some(host) = &ovr.host {
            merged.host = host.clone();
        }
        if let Some(port) = ovr.port {
            merged.port = port;
        }
        if let Some(tls) = ovr.tls {
            merged.tls = tls;
        }
        if let Some(retries) = ovr.retries {
            merged.retries = retries;
        }
    }

    if let Some(base) = base {
        if let Some(host) = &base.host {
            merged.host = host.clone();
        }
        if let Some(port) = base.port {
            merged.port = port;
        }
        if let Some(tls) = base.tls {
            merged.tls = tls;
        }
        if let Some(retries) = base.retries {
            merged.retries = retries;
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "localhost".into(),
            port: 8080,
            tls: false,
            retries: 3,
        }
    }

    #[test]
    fn override_wins_over_base_and_defaults() {
        let base = PartialConfig {
            host: Some("base.internal".into()),
            port: Some(9000),
            tls: Some(true),
            retries: None,
        };
        let ovr = PartialConfig {
            host: Some("override.internal".into()),
            port: None,
            tls: Some(false),
            retries: Some(7),
        };

        let merged = merge_config(&defaults(), Some(&base), Some(&ovr));

        assert_eq!(
            merged,
            Config {
                host: "override.internal".into(),
                port: 9000,
                tls: false,
                retries: 7,
            }
        );
    }

    #[test]
    fn base_fills_only_missing_override_fields() {
        let base = PartialConfig {
            host: None,
            port: Some(7000),
            tls: Some(true),
            retries: Some(5),
        };
        let ovr = PartialConfig {
            host: Some("service.prod".into()),
            port: Some(7443),
            tls: None,
            retries: None,
        };

        let merged = merge_config(&defaults(), Some(&base), Some(&ovr));

        assert_eq!(merged.host, "service.prod");
        assert_eq!(merged.port, 7443);
        assert_eq!(merged.tls, true);
        assert_eq!(merged.retries, 5);
    }

    #[test]
    fn defaults_remain_when_not_provided_elsewhere() {
        let base = PartialConfig {
            host: None,
            port: None,
            tls: Some(true),
            retries: None,
        };

        let merged = merge_config(&defaults(), Some(&base), None);

        assert_eq!(merged.host, "localhost");
        assert_eq!(merged.port, 8080);
        assert_eq!(merged.tls, true);
        assert_eq!(merged.retries, 3);
    }
}
