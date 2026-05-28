#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub use_tls: bool,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub use_tls: Option<bool>,
    pub timeout_ms: Option<u64>,
}

pub fn merge_config(
    defaults: &PartialConfig,
    env: &PartialConfig,
    local: &PartialConfig,
) -> Config {
    Config {
        host: defaults
            .host
            .clone()
            .or_else(|| env.host.clone())
            .or_else(|| local.host.clone())
            .unwrap_or_else(|| "127.0.0.1".to_string()),
        port: defaults.port.or(env.port).or(local.port).unwrap_or(8080),
        use_tls: defaults.use_tls.or(env.use_tls).or(local.use_tls).unwrap_or(false),
        timeout_ms: local
            .timeout_ms
            .or(env.timeout_ms)
            .or(defaults.timeout_ms)
            .unwrap_or(1_000),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn partial(
        host: Option<&str>,
        port: Option<u16>,
        use_tls: Option<bool>,
        timeout_ms: Option<u64>,
    ) -> PartialConfig {
        PartialConfig {
            host: host.map(str::to_string),
            port,
            use_tls,
            timeout_ms,
        }
    }

    #[test]
    fn local_overrides_env_and_defaults() {
        let defaults = partial(Some("default.internal"), Some(80), Some(false), Some(1_000));
        let env = partial(Some("env.internal"), Some(443), Some(true), Some(2_000));
        let local = partial(Some("localhost"), Some(3000), Some(false), Some(5_000));

        let merged = merge_config(&defaults, &env, &local);

        assert_eq!(
            merged,
            Config {
                host: "localhost".to_string(),
                port: 3000,
                use_tls: false,
                timeout_ms: 5_000,
            }
        );
    }

    #[test]
    fn env_fills_missing_local_values_but_not_defaults_over_env() {
        let defaults = partial(Some("default.internal"), Some(80), Some(false), Some(1_000));
        let env = partial(Some("env.internal"), Some(443), Some(true), Some(2_000));
        let local = partial(None, None, None, Some(9_000));

        let merged = merge_config(&defaults, &env, &local);

        assert_eq!(merged.host, "env.internal");
        assert_eq!(merged.port, 443);
        assert!(merged.use_tls);
        assert_eq!(merged.timeout_ms, 9_000);
    }

    #[test]
    fn defaults_only_apply_when_higher_layers_are_missing() {
        let defaults = partial(Some("default.internal"), Some(80), Some(true), Some(1_500));
        let env = partial(None, None, None, None);
        let local = partial(None, Some(9000), None, None);

        let merged = merge_config(&defaults, &env, &local);

        assert_eq!(merged.host, "default.internal");
        assert_eq!(merged.port, 9000);
        assert!(merged.use_tls);
        assert_eq!(merged.timeout_ms, 1_500);
    }

    #[test]
    fn hardcoded_fallbacks_are_used_when_all_layers_are_empty() {
        let empty = partial(None, None, None, None);
        let merged = merge_config(&empty, &empty, &empty);

        assert_eq!(
            merged,
            Config {
                host: "127.0.0.1".to_string(),
                port: 8080,
                use_tls: false,
                timeout_ms: 1_000,
            }
        );
    }
}
