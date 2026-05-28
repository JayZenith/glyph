#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub tls: bool,
    pub features: Vec<&'static str>,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub tls: Option<bool>,
    pub features: Option<Vec<&'static str>>,
}

pub fn merge_config(defaults: PartialConfig, env: PartialConfig, user: PartialConfig) -> Config {
    let host = defaults
        .host
        .or(env.host)
        .or(user.host)
        .unwrap_or_else(|| "127.0.0.1".to_string());

    let port = defaults.port.or(env.port).or(user.port).unwrap_or(8080);
    let tls = defaults.tls.or(env.tls).or(user.tls).unwrap_or(false);

    let mut features = Vec::new();
    if let Some(v) = user.features {
        features = v;
    } else if let Some(v) = env.features {
        features = v;
    } else if let Some(v) = defaults.features {
        features = v;
    }

    Config {
        host,
        port,
        tls,
        features,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_values_override_env_and_defaults() {
        let defaults = PartialConfig {
            host: Some("0.0.0.0".to_string()),
            port: Some(80),
            tls: Some(false),
            features: Some(vec!["base"]),
        };
        let env = PartialConfig {
            host: Some("env.internal".to_string()),
            port: Some(443),
            tls: Some(true),
            features: Some(vec!["metrics"]),
        };
        let user = PartialConfig {
            host: Some("example.com".to_string()),
            port: None,
            tls: Some(false),
            features: Some(vec!["debug"]),
        };

        let merged = merge_config(defaults, env, user);

        assert_eq!(merged.host, "example.com");
        assert_eq!(merged.port, 443);
        assert!(!merged.tls);
        assert_eq!(merged.features, vec!["base", "metrics", "debug"]);
    }

    #[test]
    fn lower_layers_fill_missing_values_and_keep_ordered_unique_features() {
        let defaults = PartialConfig {
            host: None,
            port: Some(3000),
            tls: Some(false),
            features: Some(vec!["base", "cache"]),
        };
        let env = PartialConfig {
            host: Some("service.local".to_string()),
            port: None,
            tls: None,
            features: Some(vec!["cache", "metrics"]),
        };
        let user = PartialConfig {
            host: None,
            port: Some(9000),
            tls: None,
            features: Some(vec!["metrics", "trace"]),
        };

        let merged = merge_config(defaults, env, user);

        assert_eq!(merged.host, "service.local");
        assert_eq!(merged.port, 9000);
        assert!(!merged.tls);
        assert_eq!(merged.features, vec!["base", "cache", "metrics", "trace"]);
    }

    #[test]
    fn falls_back_to_builtin_defaults_when_layers_absent() {
        let merged = merge_config(PartialConfig::default(), PartialConfig::default(), PartialConfig::default());

        assert_eq!(merged.host, "127.0.0.1");
        assert_eq!(merged.port, 8080);
        assert!(!merged.tls);
        assert!(merged.features.is_empty());
    }
}
