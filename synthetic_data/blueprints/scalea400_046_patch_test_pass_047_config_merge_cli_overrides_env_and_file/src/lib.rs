#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub debug: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub debug: Option<bool>,
}

pub fn merge_config(
    defaults: Config,
    file: PartialConfig,
    env: PartialConfig,
    cli: PartialConfig,
) -> Config {
    let mut merged = defaults;

    if let Some(host) = file.host {
        merged.host = host;
    }
    if let Some(port) = file.port {
        merged.port = port;
    }
    if let Some(debug) = file.debug {
        merged.debug = debug;
    }

    if let Some(host) = env.host {
        if !host.is_empty() {
            merged.host = host;
        }
    }
    if let Some(port) = env.port {
        merged.port = port;
    }
    if let Some(debug) = env.debug {
        merged.debug = debug;
    }

    if let Some(host) = cli.host {
        if !host.is_empty() {
            merged.host = host;
        }
    }
    if let Some(port) = cli.port {
        if port != 0 {
            merged.port = port;
        }
    }
    if let Some(debug) = cli.debug {
        if debug {
            merged.debug = true;
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "localhost".into(),
            port: 8080,
            debug: false,
        }
    }

    #[test]
    fn file_values_override_defaults() {
        let merged = merge_config(
            defaults(),
            PartialConfig {
                host: Some("file-host".into()),
                port: Some(9000),
                debug: Some(true),
            },
            PartialConfig::default(),
            PartialConfig::default(),
        );

        assert_eq!(
            merged,
            Config {
                host: "file-host".into(),
                port: 9000,
                debug: true,
            }
        );
    }

    #[test]
    fn env_overrides_file_but_empty_host_is_ignored() {
        let merged = merge_config(
            defaults(),
            PartialConfig {
                host: Some("file-host".into()),
                port: Some(7000),
                debug: Some(false),
            },
            PartialConfig {
                host: Some(String::new()),
                port: Some(7100),
                debug: Some(true),
            },
            PartialConfig::default(),
        );

        assert_eq!(merged.host, "file-host");
        assert_eq!(merged.port, 7100);
        assert!(merged.debug);
    }

    #[test]
    fn cli_overrides_env_even_with_false_and_zero_like_values() {
        let merged = merge_config(
            defaults(),
            PartialConfig {
                host: Some("file-host".into()),
                port: Some(5000),
                debug: Some(false),
            },
            PartialConfig {
                host: Some("env-host".into()),
                port: Some(6000),
                debug: Some(true),
            },
            PartialConfig {
                host: Some("cli-host".into()),
                port: Some(0),
                debug: Some(false),
            },
        );

        assert_eq!(
            merged,
            Config {
                host: "cli-host".into(),
                port: 0,
                debug: false,
            }
        );
    }
}
