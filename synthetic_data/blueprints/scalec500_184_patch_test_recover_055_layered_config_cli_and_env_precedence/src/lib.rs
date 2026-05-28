#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub endpoint: String,
    pub timeout_ms: u32,
    pub retries: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            endpoint: "https://default.service".to_string(),
            timeout_ms: 1000,
            retries: 2,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub endpoint: Option<String>,
    pub timeout_ms: Option<u32>,
    pub retries: Option<u8>,
}

pub fn merge_config(
    defaults: Config,
    file: Option<PartialConfig>,
    env: Option<PartialConfig>,
    cli: Option<PartialConfig>,
) -> Config {
    let mut cfg = defaults;

    if let Some(layer) = file {
        if let Some(v) = layer.endpoint {
            cfg.endpoint = v;
        }
        if let Some(v) = layer.timeout_ms {
            cfg.timeout_ms = v;
        }
        if let Some(v) = layer.retries {
            cfg.retries = v;
        }
    }

    if let Some(layer) = env {
        if let Some(v) = layer.endpoint {
            cfg.endpoint = v;
        }
        if let Some(v) = layer.timeout_ms {
            cfg.timeout_ms = v;
        }
        if let Some(v) = layer.retries {
            cfg.retries = v;
        }
    }

    if let Some(layer) = cli {
        if let Some(v) = layer.endpoint {
            cfg.endpoint = v;
        }
        if let Some(v) = layer.timeout_ms {
            cfg.timeout_ms = v;
        }
        if let Some(v) = layer.retries {
            cfg.retries = v;
        }
    }

    if cfg.timeout_ms == 0 {
        cfg.timeout_ms = defaults.timeout_ms;
    }

    if cfg.retries == 0 {
        cfg.retries = defaults.retries;
    }

    cfg
}

#[cfg(test)]
mod tests {
    use super::*;

    fn partial(endpoint: Option<&str>, timeout_ms: Option<u32>, retries: Option<u8>) -> PartialConfig {
        PartialConfig {
            endpoint: endpoint.map(|s| s.to_string()),
            timeout_ms,
            retries,
        }
    }

    #[test]
    fn higher_precedence_layers_override_lower_ones() {
        let defaults = Config::default();
        let file = Some(partial(Some("https://file.service"), Some(2000), Some(4)));
        let env = Some(partial(None, Some(3000), Some(5)));
        let cli = Some(partial(Some("https://cli.service"), None, Some(6)));

        let cfg = merge_config(defaults, file, env, cli);

        assert_eq!(cfg.endpoint, "https://cli.service");
        assert_eq!(cfg.timeout_ms, 3000);
        assert_eq!(cfg.retries, 6);
    }

    #[test]
    fn zero_timeout_and_retries_from_overrides_are_explicit() {
        let defaults = Config::default();
        let file = Some(partial(None, Some(2500), Some(3)));
        let env = Some(partial(None, Some(0), Some(0)));
        let cli = None;

        let cfg = merge_config(defaults, file, env, cli);

        assert_eq!(cfg.timeout_ms, 0);
        assert_eq!(cfg.retries, 0);
    }

    #[test]
    fn missing_values_fall_back_through_layers() {
        let defaults = Config::default();
        let file = Some(partial(Some("https://file.service"), None, None));
        let env = Some(partial(None, Some(1800), None));
        let cli = Some(partial(None, None, Some(9)));

        let cfg = merge_config(defaults, file, env, cli);

        assert_eq!(cfg.endpoint, "https://file.service");
        assert_eq!(cfg.timeout_ms, 1800);
        assert_eq!(cfg.retries, 9);
    }
}
