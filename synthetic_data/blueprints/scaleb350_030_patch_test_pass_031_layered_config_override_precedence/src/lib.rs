#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialConfig {
    pub endpoint: Option<String>,
    pub retries: Option<u32>,
    pub use_tls: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub endpoint: String,
    pub retries: u32,
    pub use_tls: bool,
}

pub fn merge_config(defaults: &PartialConfig, env: &PartialConfig, user: &PartialConfig) -> Config {
    let endpoint = defaults
        .endpoint
        .clone()
        .or_else(|| env.endpoint.clone())
        .or_else(|| user.endpoint.clone())
        .unwrap_or_else(|| "http://localhost".to_string());

    let retries = user
        .retries
        .filter(|v| *v > 0)
        .or(env.retries.filter(|v| *v > 0))
        .or(defaults.retries)
        .unwrap_or(3);

    let use_tls = user.use_tls.unwrap_or(env.use_tls.unwrap_or(defaults.use_tls.unwrap_or(true)));

    Config {
        endpoint,
        retries,
        use_tls,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn partial(endpoint: Option<&str>, retries: Option<u32>, use_tls: Option<bool>) -> PartialConfig {
        PartialConfig {
            endpoint: endpoint.map(str::to_string),
            retries,
            use_tls,
        }
    }

    #[test]
    fn highest_precedence_endpoint_wins() {
        let defaults = partial(Some("http://default"), Some(2), Some(true));
        let env = partial(Some("http://env"), Some(4), Some(true));
        let user = partial(Some("http://user"), None, None);

        let cfg = merge_config(&defaults, &env, &user);
        assert_eq!(cfg.endpoint, "http://user");
        assert_eq!(cfg.retries, 4);
        assert!(cfg.use_tls);
    }

    #[test]
    fn explicit_false_from_user_must_override_lower_layers() {
        let defaults = partial(Some("http://default"), Some(2), Some(true));
        let env = partial(None, Some(5), Some(true));
        let user = partial(None, None, Some(false));

        let cfg = merge_config(&defaults, &env, &user);
        assert!(!cfg.use_tls);
        assert_eq!(cfg.retries, 5);
    }

    #[test]
    fn explicit_zero_retries_is_a_valid_override() {
        let defaults = partial(Some("http://default"), Some(3), Some(true));
        let env = partial(None, Some(2), None);
        let user = partial(None, Some(0), None);

        let cfg = merge_config(&defaults, &env, &user);
        assert_eq!(cfg.retries, 0);
    }

    #[test]
    fn falls_back_when_all_layers_missing() {
        let defaults = partial(None, None, None);
        let env = partial(None, None, None);
        let user = partial(None, None, None);

        let cfg = merge_config(&defaults, &env, &user);
        assert_eq!(cfg.endpoint, "http://localhost");
        assert_eq!(cfg.retries, 3);
        assert!(cfg.use_tls);
    }
}
