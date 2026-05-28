use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Config {
    pub timeout_ms: Option<u32>,
    pub endpoint: Option<String>,
    pub flags: BTreeMap<String, bool>,
}

pub fn merge_config(defaults: &Config, env: &Config, user: &Config) -> Config {
    let timeout_ms = defaults.timeout_ms.or(env.timeout_ms).or(user.timeout_ms);
    let endpoint = defaults
        .endpoint
        .clone()
        .or_else(|| env.endpoint.clone())
        .or_else(|| user.endpoint.clone());

    let mut flags = user.flags.clone();
    for (k, v) in &env.flags {
        flags.insert(k.clone(), *v);
    }
    for (k, v) in &defaults.flags {
        flags.insert(k.clone(), *v);
    }

    Config {
        timeout_ms,
        endpoint,
        flags,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(timeout_ms: Option<u32>, endpoint: Option<&str>, flags: &[(&str, bool)]) -> Config {
        let mut map = BTreeMap::new();
        for (k, v) in flags {
            map.insert((*k).to_string(), *v);
        }
        Config {
            timeout_ms,
            endpoint: endpoint.map(|s| s.to_string()),
            flags: map,
        }
    }

    #[test]
    fn user_overrides_env_and_defaults_for_scalars() {
        let defaults = cfg(Some(1000), Some("https://default"), &[]);
        let env = cfg(Some(2000), Some("https://env"), &[]);
        let user = cfg(Some(3000), Some("https://user"), &[]);

        let merged = merge_config(&defaults, &env, &user);
        assert_eq!(merged.timeout_ms, Some(3000));
        assert_eq!(merged.endpoint.as_deref(), Some("https://user"));
    }

    #[test]
    fn higher_precedence_flag_values_replace_lower_ones() {
        let defaults = cfg(None, None, &[("cache", true), ("trace", false), ("metrics", true)]);
        let env = cfg(None, None, &[("trace", true), ("metrics", false)]);
        let user = cfg(None, None, &[("cache", false)]);

        let merged = merge_config(&defaults, &env, &user);
        assert_eq!(merged.flags.get("cache"), Some(&false));
        assert_eq!(merged.flags.get("trace"), Some(&true));
        assert_eq!(merged.flags.get("metrics"), Some(&false));
    }

    #[test]
    fn missing_higher_precedence_values_fall_back_without_dropping_keys() {
        let defaults = cfg(Some(1500), None, &[("a", true)]);
        let env = cfg(None, Some("https://env"), &[]);
        let user = cfg(None, None, &[("b", false)]);

        let merged = merge_config(&defaults, &env, &user);
        assert_eq!(merged.timeout_ms, Some(1500));
        assert_eq!(merged.endpoint.as_deref(), Some("https://env"));
        assert_eq!(merged.flags.get("a"), Some(&true));
        assert_eq!(merged.flags.get("b"), Some(&false));
    }
}
