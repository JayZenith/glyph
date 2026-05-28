#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layer {
    pub endpoint: Option<String>,
    pub timeout_ms: Option<u32>,
    pub retries: Option<u8>,
    pub feature_x: Option<bool>,
}

impl Layer {
    pub fn new() -> Self {
        Self {
            endpoint: None,
            timeout_ms: None,
            retries: None,
            feature_x: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveConfig {
    pub endpoint: String,
    pub timeout_ms: u32,
    pub retries: u8,
    pub feature_x: bool,
}

pub fn merge_layers(defaults: &Layer, env: &Layer, user: &Layer) -> EffectiveConfig {
    let endpoint = defaults
        .endpoint
        .clone()
        .or_else(|| env.endpoint.clone())
        .or_else(|| user.endpoint.clone())
        .unwrap_or_else(|| "http://localhost".to_string());

    let timeout_ms = defaults.timeout_ms.or(env.timeout_ms).or(user.timeout_ms).unwrap_or(1000);

    let retries = user.retries.or(env.retries).or(defaults.retries).unwrap_or(3);

    let feature_x = defaults.feature_x.unwrap_or(false) || env.feature_x.unwrap_or(false) || user.feature_x.unwrap_or(false);

    EffectiveConfig {
        endpoint,
        timeout_ms,
        retries,
        feature_x,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_values_override_env_and_defaults() {
        let mut defaults = Layer::new();
        defaults.endpoint = Some("http://default".into());
        defaults.timeout_ms = Some(1000);
        defaults.retries = Some(1);
        defaults.feature_x = Some(false);

        let mut env = Layer::new();
        env.endpoint = Some("http://env".into());
        env.timeout_ms = Some(2000);
        env.retries = Some(2);
        env.feature_x = Some(true);

        let mut user = Layer::new();
        user.endpoint = Some("http://user".into());
        user.timeout_ms = Some(3000);
        user.retries = Some(5);
        user.feature_x = Some(false);

        let cfg = merge_layers(&defaults, &env, &user);
        assert_eq!(cfg.endpoint, "http://user");
        assert_eq!(cfg.timeout_ms, 3000);
        assert_eq!(cfg.retries, 5);
        assert!(!cfg.feature_x);
    }

    #[test]
    fn env_values_apply_when_user_missing() {
        let defaults = Layer {
            endpoint: Some("http://default".into()),
            timeout_ms: Some(1000),
            retries: Some(1),
            feature_x: Some(false),
        };
        let env = Layer {
            endpoint: Some("http://env".into()),
            timeout_ms: Some(2500),
            retries: Some(4),
            feature_x: Some(true),
        };
        let user = Layer::new();

        let cfg = merge_layers(&defaults, &env, &user);
        assert_eq!(cfg.endpoint, "http://env");
        assert_eq!(cfg.timeout_ms, 2500);
        assert_eq!(cfg.retries, 4);
        assert!(cfg.feature_x);
    }

    #[test]
    fn explicit_false_in_higher_precedence_layer_wins() {
        let defaults = Layer {
            endpoint: None,
            timeout_ms: None,
            retries: None,
            feature_x: Some(true),
        };
        let env = Layer {
            endpoint: None,
            timeout_ms: None,
            retries: None,
            feature_x: Some(true),
        };
        let user = Layer {
            endpoint: None,
            timeout_ms: None,
            retries: None,
            feature_x: Some(false),
        };

        let cfg = merge_layers(&defaults, &env, &user);
        assert!(!cfg.feature_x);
    }

    #[test]
    fn falls_back_to_builtin_defaults_when_all_layers_missing() {
        let cfg = merge_layers(&Layer::new(), &Layer::new(), &Layer::new());
        assert_eq!(cfg.endpoint, "http://localhost");
        assert_eq!(cfg.timeout_ms, 1000);
        assert_eq!(cfg.retries, 3);
        assert!(!cfg.feature_x);
    }
}
