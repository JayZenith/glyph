use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub debug: bool,
    pub timeout_ms: u64,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub port: Option<u16>,
    pub host: Option<String>,
    pub debug: Option<bool>,
    pub timeout_ms: Option<u64>,
    pub headers: HashMap<String, String>,
}

impl Config {
    pub fn defaults() -> Self {
        let mut headers = HashMap::new();
        headers.insert("x-mode".to_string(), "safe".to_string());
        headers.insert("x-region".to_string(), "global".to_string());
        Self {
            port: 8080,
            host: "127.0.0.1".to_string(),
            debug: false,
            timeout_ms: 1000,
            headers,
        }
    }
}

pub fn merge_config(base: &Config, env: &PartialConfig, cli: &PartialConfig) -> Config {
    let mut merged = base.clone();

    if let Some(port) = env.port {
        merged.port = port;
    }
    if let Some(port) = cli.port {
        merged.port = port;
    }

    if let Some(host) = env.host.as_ref() {
        merged.host = host.clone();
    }
    if let Some(host) = cli.host.as_ref() {
        merged.host = host.clone();
    }

    if let Some(debug) = env.debug {
        merged.debug = debug;
    }
    if let Some(debug) = cli.debug {
        merged.debug = debug;
    }

    if let Some(timeout_ms) = env.timeout_ms {
        merged.timeout_ms = timeout_ms;
    }
    if let Some(timeout_ms) = cli.timeout_ms {
        merged.timeout_ms = timeout_ms;
    }

    for (k, v) in &env.headers {
        merged.headers.insert(k.clone(), v.clone());
    }
    for (k, v) in &cli.headers {
        merged.headers.insert(k.clone(), v.clone());
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    fn partial() -> PartialConfig {
        PartialConfig::default()
    }

    #[test]
    fn cli_values_override_env_and_defaults() {
        let base = Config::defaults();

        let mut env = partial();
        env.port = Some(9000);
        env.host = Some("env.local".to_string());
        env.debug = Some(true);
        env.timeout_ms = Some(2500);
        env.headers.insert("x-mode".to_string(), "env".to_string());
        env.headers.insert("x-env-only".to_string(), "yes".to_string());

        let mut cli = partial();
        cli.port = Some(7000);
        cli.debug = Some(false);
        cli.headers.insert("x-mode".to_string(), "cli".to_string());
        cli.headers.insert("x-cli-only".to_string(), "1".to_string());

        let merged = merge_config(&base, &env, &cli);

        assert_eq!(merged.port, 7000);
        assert_eq!(merged.host, "env.local");
        assert!(!merged.debug);
        assert_eq!(merged.timeout_ms, 2500);
        assert_eq!(merged.headers.get("x-mode"), Some(&"cli".to_string()));
        assert_eq!(merged.headers.get("x-region"), Some(&"global".to_string()));
        assert_eq!(merged.headers.get("x-env-only"), Some(&"yes".to_string()));
        assert_eq!(merged.headers.get("x-cli-only"), Some(&"1".to_string()));
    }

    #[test]
    fn defaults_are_used_when_no_overrides_exist() {
        let base = Config::defaults();
        let env = partial();
        let cli = partial();

        let merged = merge_config(&base, &env, &cli);

        assert_eq!(merged, base);
    }

    #[test]
    fn env_values_apply_when_cli_is_missing() {
        let base = Config::defaults();
        let mut env = partial();
        env.host = Some("10.0.0.5".to_string());
        env.timeout_ms = Some(4000);
        env.headers.insert("x-region".to_string(), "eu".to_string());

        let cli = partial();
        let merged = merge_config(&base, &env, &cli);

        assert_eq!(merged.port, 8080);
        assert_eq!(merged.host, "10.0.0.5");
        assert_eq!(merged.timeout_ms, 4000);
        assert_eq!(merged.headers.get("x-region"), Some(&"eu".to_string()));
        assert_eq!(merged.headers.get("x-mode"), Some(&"safe".to_string()));
    }
}
