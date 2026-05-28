#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub use_tls: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub use_tls: Option<bool>,
}

pub fn merge_config(
    defaults: &Config,
    env: Option<&PartialConfig>,
    cli: Option<&PartialConfig>,
) -> Config {
    let mut out = defaults.clone();

    if let Some(layer) = env {
        if let Some(host) = &layer.host {
            out.host = host.clone();
        }
        if let Some(port) = layer.port {
            out.port = port;
        }
        if let Some(use_tls) = layer.use_tls {
            out.use_tls = use_tls;
        }
    }

    if let Some(layer) = cli {
        if let Some(host) = &layer.host {
            out.host = host.clone();
        }
        if let Some(port) = layer.port {
            out.port = port;
        }
        if let Some(use_tls) = layer.use_tls {
            out.use_tls = use_tls;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            use_tls: false,
        }
    }

    #[test]
    fn keeps_defaults_when_no_layers_present() {
        let merged = merge_config(&defaults(), None, None);
        assert_eq!(merged, defaults());
    }

    #[test]
    fn env_overrides_only_fields_it_sets() {
        let env = PartialConfig {
            host: Some("env.internal".to_string()),
            port: None,
            use_tls: Some(true),
        };

        let merged = merge_config(&defaults(), Some(&env), None);

        assert_eq!(
            merged,
            Config {
                host: "env.internal".to_string(),
                port: 8080,
                use_tls: true,
            }
        );
    }

    #[test]
    fn cli_takes_precedence_over_env() {
        let env = PartialConfig {
            host: Some("env.internal".to_string()),
            port: Some(9000),
            use_tls: Some(false),
        };
        let cli = PartialConfig {
            host: None,
            port: Some(7000),
            use_tls: Some(true),
        };

        let merged = merge_config(&defaults(), Some(&env), Some(&cli));

        assert_eq!(
            merged,
            Config {
                host: "env.internal".to_string(),
                port: 7000,
                use_tls: true,
            }
        );
    }
}
