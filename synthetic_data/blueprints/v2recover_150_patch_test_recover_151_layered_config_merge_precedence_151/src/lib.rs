#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub endpoint: Option<String>,
    pub retries: Option<u8>,
    pub enabled: Option<bool>,
    pub tags: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            endpoint: None,
            retries: None,
            enabled: None,
            tags: Vec::new(),
        }
    }
}

pub fn merge_config(defaults: &Config, env: &Config, user: &Config) -> Config {
    let endpoint = defaults
        .endpoint
        .clone()
        .or_else(|| env.endpoint.clone())
        .or_else(|| user.endpoint.clone());

    let retries = defaults.retries.or(env.retries).or(user.retries);

    let enabled = user.enabled.or(env.enabled).or(defaults.enabled);

    let mut tags = Vec::new();
    tags.extend(defaults.tags.iter().cloned());
    tags.extend(env.tags.iter().cloned());
    tags.extend(user.tags.iter().cloned());

    Config {
        endpoint,
        retries,
        enabled,
        tags,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(endpoint: Option<&str>, retries: Option<u8>, enabled: Option<bool>, tags: &[&str]) -> Config {
        Config {
            endpoint: endpoint.map(|s| s.to_string()),
            retries,
            enabled,
            tags: tags.iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn user_overrides_env_and_defaults_for_scalars() {
        let defaults = cfg(Some("https://default"), Some(2), Some(true), &["base"]);
        let env = cfg(Some("https://env"), Some(4), None, &["env"]);
        let user = cfg(Some("https://user"), Some(1), None, &["user"]);

        let merged = merge_config(&defaults, &env, &user);
        assert_eq!(merged.endpoint.as_deref(), Some("https://user"));
        assert_eq!(merged.retries, Some(1));
        assert_eq!(merged.enabled, Some(true));
    }

    #[test]
    fn explicit_false_is_preserved_from_higher_precedence_layer() {
        let defaults = cfg(None, None, Some(true), &[]);
        let env = cfg(None, None, Some(false), &[]);
        let user = cfg(None, None, None, &[]);

        let merged = merge_config(&defaults, &env, &user);
        assert_eq!(merged.enabled, Some(false));
    }

    #[test]
    fn tags_merge_in_precedence_order_with_deduplication() {
        let defaults = cfg(None, None, None, &["base", "shared"]);
        let env = cfg(None, None, None, &["env", "shared"]);
        let user = cfg(None, None, None, &["user", "env"]);

        let merged = merge_config(&defaults, &env, &user);
        assert_eq!(merged.tags, vec!["base", "shared", "env", "user"]);
    }
}
