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
    direct: Option<&PartialConfig>,
) -> Config {
    let host = direct
        .and_then(|c| c.host.clone())
        .or_else(|| profile.and_then(|c| c.host.clone()))
        .unwrap_or_else(|| defaults.host.clone());

    let port = profile
        .and_then(|c| c.port)
        .or_else(|| direct.and_then(|c| c.port))
        .unwrap_or(defaults.port);

    let tls = direct
        .and_then(|c| c.tls)
        .or_else(|| profile.and_then(|c| c.tls))
        .unwrap_or(defaults.tls);

    Config { host, port, tls }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            tls: false,
        }
    }

    #[test]
    fn direct_values_override_profile_values() {
        let profile = PartialConfig {
            host: Some("profile.internal".to_string()),
            port: Some(7000),
            tls: Some(true),
        };
        let direct = PartialConfig {
            host: Some("cli.example.com".to_string()),
            port: Some(9000),
            tls: None,
        };

        let merged = merge_config(&defaults(), Some(&profile), Some(&direct));

        assert_eq!(merged.host, "cli.example.com");
        assert_eq!(merged.port, 9000);
        assert_eq!(merged.tls, true);
    }

    #[test]
    fn missing_direct_field_falls_back_to_profile_then_default() {
        let profile = PartialConfig {
            host: None,
            port: Some(7000),
            tls: Some(true),
        };
        let direct = PartialConfig {
            host: None,
            port: None,
            tls: None,
        };

        let merged = merge_config(&defaults(), Some(&profile), Some(&direct));

        assert_eq!(merged.host, "localhost");
        assert_eq!(merged.port, 7000);
        assert_eq!(merged.tls, true);
    }

    #[test]
    fn defaults_are_used_when_no_overrides_exist() {
        let merged = merge_config(&defaults(), None, None);
        assert_eq!(merged, defaults());
    }
}
