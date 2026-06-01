#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub use_tls: bool,
    pub timeout_ms: u32,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub use_tls: Option<bool>,
    pub timeout_ms: Option<u32>,
    pub tags: Option<Vec<String>>,
}

pub fn merge_config(
    defaults: PartialConfig,
    base: PartialConfig,
    env: PartialConfig,
    cli: PartialConfig,
) -> Config {
    let mut cfg = Config {
        host: "localhost".to_string(),
        port: 80,
        use_tls: false,
        timeout_ms: 1000,
        tags: Vec::new(),
    };

    for layer in [defaults, base, env, cli] {
        if let Some(host) = layer.host {
            cfg.host = host;
        }
        if let Some(port) = layer.port {
            cfg.port = port;
        }
        if let Some(use_tls) = layer.use_tls {
            cfg.use_tls = use_tls;
        }
        if let Some(timeout_ms) = layer.timeout_ms {
            cfg.timeout_ms = timeout_ms;
        }
        if let Some(tags) = layer.tags {
            cfg.tags.extend(tags);
        }
    }

    cfg
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pc(
        host: Option<&str>,
        port: Option<u16>,
        use_tls: Option<bool>,
        timeout_ms: Option<u32>,
        tags: Option<Vec<&str>>,
    ) -> PartialConfig {
        PartialConfig {
            host: host.map(|s| s.to_string()),
            port,
            use_tls,
            timeout_ms,
            tags: tags.map(|v| v.into_iter().map(|s| s.to_string()).collect()),
        }
    }

    #[test]
    fn later_layers_override_scalar_fields() {
        let cfg = merge_config(
            pc(Some("def.local"), Some(8080), Some(false), Some(1000), None),
            pc(Some("base.local"), None, Some(true), Some(1500), None),
            pc(None, Some(9000), None, None, None),
            pc(Some("cli.local"), None, Some(false), None, None),
        );

        assert_eq!(cfg.host, "cli.local");
        assert_eq!(cfg.port, 9000);
        assert!(!cfg.use_tls);
        assert_eq!(cfg.timeout_ms, 1500);
    }

    #[test]
    fn cli_zero_timeout_means_keep_previous_non_zero() {
        let cfg = merge_config(
            pc(None, None, None, Some(1000), None),
            pc(None, None, None, Some(2500), None),
            pc(None, None, None, None, None),
            pc(None, None, None, Some(0), None),
        );

        assert_eq!(cfg.timeout_ms, 2500);
    }

    #[test]
    fn tags_are_replaced_by_latest_non_empty_and_empty_clears() {
        let cfg = merge_config(
            pc(None, None, None, None, Some(vec!["default", "shared"])),
            pc(None, None, None, None, Some(vec!["base"])),
            pc(None, None, None, None, Some(vec![])),
            pc(None, None, None, None, Some(vec!["cli", "debug"])),
        );

        assert_eq!(cfg.tags, vec!["cli", "debug"]);

        let cleared = merge_config(
            pc(None, None, None, None, Some(vec!["default"])),
            pc(None, None, None, None, Some(vec!["base"])),
            pc(None, None, None, None, Some(vec!["env"])),
            pc(None, None, None, None, Some(vec![])),
        );

        assert!(cleared.tags.is_empty());
    }
}
