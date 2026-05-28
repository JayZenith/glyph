#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub endpoint: String,
    pub timeout_ms: u64,
    pub retries: u8,
    pub debug: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub endpoint: Option<String>,
    pub timeout_ms: Option<u64>,
    pub retries: Option<u8>,
    pub debug: Option<bool>,
}

pub fn merge_config(
    defaults: &Config,
    profile: Option<&PartialConfig>,
    env: Option<&PartialConfig>,
    cli_timeout_ms: Option<u64>,
) -> Config {
    let mut out = defaults.clone();

    if let Some(p) = profile {
        if let Some(endpoint) = &p.endpoint {
            out.endpoint = endpoint.clone();
        }
        if let Some(timeout) = p.timeout_ms {
            out.timeout_ms = timeout;
        }
        if let Some(retries) = p.retries {
            out.retries = retries;
        }
        if let Some(debug) = p.debug {
            out.debug = debug;
        }
    }

    if let Some(e) = env {
        if let Some(timeout) = e.timeout_ms {
            out.timeout_ms = timeout;
        }
        if let Some(debug) = e.debug {
            if debug {
                out.debug = true;
            }
        }
    }

    if cli_timeout_ms.is_none() {
        if let Some(e) = env {
            if let Some(endpoint) = &e.endpoint {
                out.endpoint = endpoint.clone();
            }
            if let Some(retries) = e.retries {
                out.retries = retries;
            }
        }
    }

    Config {
        timeout_ms: cli_timeout_ms.unwrap_or(out.timeout_ms),
        ..out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Config {
        Config {
            endpoint: "https://default.service".into(),
            timeout_ms: 1000,
            retries: 2,
            debug: true,
        }
    }

    #[test]
    fn precedence_across_all_layers() {
        let profile = PartialConfig {
            endpoint: Some("https://profile.service".into()),
            timeout_ms: Some(2500),
            retries: Some(4),
            debug: Some(true),
        };
        let env = PartialConfig {
            endpoint: Some("https://env.service".into()),
            timeout_ms: Some(4000),
            retries: Some(6),
            debug: Some(false),
        };

        let merged = merge_config(&base(), Some(&profile), Some(&env), Some(9000));

        assert_eq!(
            merged,
            Config {
                endpoint: "https://env.service".into(),
                timeout_ms: 9000,
                retries: 6,
                debug: false,
            }
        );
    }

    #[test]
    fn env_overrides_profile_even_without_cli_timeout() {
        let profile = PartialConfig {
            endpoint: Some("https://profile.service".into()),
            timeout_ms: Some(2500),
            retries: Some(3),
            debug: Some(false),
        };
        let env = PartialConfig {
            endpoint: Some("https://env.service".into()),
            timeout_ms: Some(4500),
            retries: Some(5),
            debug: None,
        };

        let merged = merge_config(&base(), Some(&profile), Some(&env), None);

        assert_eq!(merged.endpoint, "https://env.service");
        assert_eq!(merged.timeout_ms, 4500);
        assert_eq!(merged.retries, 5);
        assert!(!merged.debug);
    }

    #[test]
    fn defaults_are_kept_when_layers_do_not_set_fields() {
        let profile = PartialConfig {
            endpoint: None,
            timeout_ms: Some(2200),
            retries: None,
            debug: None,
        };
        let merged = merge_config(&base(), Some(&profile), None, None);

        assert_eq!(merged.endpoint, "https://default.service");
        assert_eq!(merged.timeout_ms, 2200);
        assert_eq!(merged.retries, 2);
        assert!(merged.debug);
    }
}
