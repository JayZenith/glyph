#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub tls: bool,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub tls: Option<bool>,
    pub timeout_ms: Option<u64>,
}

impl Config {
    pub fn defaults() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            tls: false,
            timeout_ms: 1_000,
        }
    }
}

pub fn merge_config(
    defaults: Config,
    file: Option<PartialConfig>,
    env: Option<PartialConfig>,
    cli: Option<PartialConfig>,
) -> Config {
    let mut out = defaults;

    for layer in [file, env, cli] {
        if let Some(layer) = layer {
            if let Some(host) = layer.host {
                out.host = host;
            }
            if let Some(port) = layer.port {
                out.port = port;
            }
            if let Some(tls) = layer.tls {
                out.tls = tls;
            }
            if let Some(timeout_ms) = layer.timeout_ms {
                out.timeout_ms = timeout_ms;
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn partial(
        host: Option<&str>,
        port: Option<u16>,
        tls: Option<bool>,
        timeout_ms: Option<u64>,
    ) -> PartialConfig {
        PartialConfig {
            host: host.map(str::to_string),
            port,
            tls,
            timeout_ms,
        }
    }

    #[test]
    fn later_layers_override_earlier_ones() {
        let merged = merge_config(
            Config::defaults(),
            Some(partial(Some("file.local"), Some(9000), Some(false), Some(2_000))),
            Some(partial(None, Some(9100), Some(true), None)),
            Some(partial(Some("cli.local"), None, None, Some(5_000))),
        );

        assert_eq!(
            merged,
            Config {
                host: "cli.local".to_string(),
                port: 9100,
                tls: true,
                timeout_ms: 5_000,
            }
        );
    }

    #[test]
    fn missing_values_do_not_clear_previous_layers() {
        let merged = merge_config(
            Config::defaults(),
            Some(partial(Some("cfg.local"), None, Some(true), None)),
            Some(partial(None, Some(7000), None, None)),
            Some(partial(None, None, None, Some(3_500))),
        );

        assert_eq!(merged.host, "cfg.local");
        assert_eq!(merged.port, 7000);
        assert!(merged.tls);
        assert_eq!(merged.timeout_ms, 3_500);
    }

    #[test]
    fn defaults_are_used_when_no_layer_sets_a_field() {
        let merged = merge_config(
            Config::defaults(),
            Some(partial(None, None, None, None)),
            None,
            Some(partial(None, Some(8081), None, None)),
        );

        assert_eq!(merged.host, "127.0.0.1");
        assert_eq!(merged.port, 8081);
        assert!(!merged.tls);
        assert_eq!(merged.timeout_ms, 1_000);
    }
}
