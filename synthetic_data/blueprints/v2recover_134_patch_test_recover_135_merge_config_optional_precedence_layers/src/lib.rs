#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub endpoint: Option<String>,
    pub timeout_ms: Option<u64>,
    pub retries: Option<u8>,
    pub features: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            endpoint: None,
            timeout_ms: None,
            retries: None,
            features: Vec::new(),
        }
    }
}

pub fn merge_config(defaults: &Config, env: &Config, user: &Config) -> Config {
    let endpoint = defaults
        .endpoint
        .clone()
        .or_else(|| env.endpoint.clone())
        .or_else(|| user.endpoint.clone());

    let timeout_ms = user.timeout_ms.or(env.timeout_ms).or(defaults.timeout_ms);

    let retries = user.retries.unwrap_or(defaults.retries.unwrap_or(3));

    let features = user.features.clone();

    Config {
        endpoint,
        timeout_ms,
        retries: Some(retries),
        features,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &str) -> String {
        v.to_string()
    }

    #[test]
    fn precedence_is_user_then_env_then_defaults() {
        let defaults = Config {
            endpoint: Some(s("https://default")),
            timeout_ms: Some(1000),
            retries: Some(1),
            features: vec![s("base")],
        };
        let env = Config {
            endpoint: Some(s("https://env")),
            timeout_ms: Some(2000),
            retries: Some(4),
            features: vec![s("metrics"), s("base")],
        };
        let user = Config {
            endpoint: Some(s("https://user")),
            timeout_ms: Some(5000),
            retries: Some(2),
            features: vec![s("cli"), s("metrics")],
        };

        let merged = merge_config(&defaults, &env, &user);
        assert_eq!(merged.endpoint.as_deref(), Some("https://user"));
        assert_eq!(merged.timeout_ms, Some(5000));
        assert_eq!(merged.retries, Some(2));
        assert_eq!(merged.features, vec![s("base"), s("metrics"), s("cli")]);
    }

    #[test]
    fn missing_later_values_do_not_clear_earlier_ones() {
        let defaults = Config {
            endpoint: Some(s("https://default")),
            timeout_ms: Some(1000),
            retries: Some(1),
            features: vec![s("base")],
        };
        let env = Config {
            endpoint: Some(s("https://env")),
            timeout_ms: None,
            retries: Some(4),
            features: vec![s("metrics")],
        };
        let user = Config {
            endpoint: None,
            timeout_ms: None,
            retries: None,
            features: vec![],
        };

        let merged = merge_config(&defaults, &env, &user);
        assert_eq!(merged.endpoint.as_deref(), Some("https://env"));
        assert_eq!(merged.timeout_ms, Some(1000));
        assert_eq!(merged.retries, Some(4));
        assert_eq!(merged.features, vec![s("base"), s("metrics")]);
    }

    #[test]
    fn empty_inputs_keep_defaults_and_no_duplicate_features() {
        let defaults = Config {
            endpoint: Some(s("https://default")),
            timeout_ms: Some(1500),
            retries: Some(3),
            features: vec![s("base"), s("trace")],
        };
        let env = Config::new();
        let user = Config {
            endpoint: None,
            timeout_ms: None,
            retries: None,
            features: vec![s("trace"), s("interactive")],
        };

        let merged = merge_config(&defaults, &env, &user);
        assert_eq!(merged.endpoint.as_deref(), Some("https://default"));
        assert_eq!(merged.timeout_ms, Some(1500));
        assert_eq!(merged.retries, Some(3));
        assert_eq!(merged.features, vec![s("base"), s("trace"), s("interactive")]);
    }
}
