#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub log_level: String,
    pub color: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub log_level: Option<String>,
    pub color: Option<bool>,
}

pub fn merge_config(
    defaults: &Config,
    file: Option<&PartialConfig>,
    env: Option<&PartialConfig>,
    cli: Option<&PartialConfig>,
) -> Config {
    let mut out = defaults.clone();

    if let Some(cli) = cli {
        if let Some(host) = &cli.host {
            out.host = host.clone();
        }
        if let Some(port) = cli.port {
            out.port = port;
        }
        if let Some(level) = &cli.log_level {
            out.log_level = level.clone();
        }
        if let Some(color) = cli.color {
            out.color = color;
        }
    }

    if let Some(env) = env {
        if let Some(host) = &env.host {
            out.host = host.clone();
        }
        if let Some(port) = env.port {
            out.port = port;
        }
        if let Some(level) = &env.log_level {
            out.log_level = level.clone();
        }
        if let Some(color) = env.color {
            out.color = color;
        }
    }

    if let Some(file) = file {
        if let Some(host) = &file.host {
            out.host = host.clone();
        }
        if let Some(port) = file.port {
            out.port = port;
        }
        if let Some(level) = &file.log_level {
            out.log_level = level.clone();
        }
        if let Some(color) = file.color {
            out.color = color;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "127.0.0.1".into(),
            port: 8080,
            log_level: "info".into(),
            color: false,
        }
    }

    #[test]
    fn precedence_is_defaults_then_file_then_env_then_cli() {
        let file = PartialConfig {
            host: Some("file.local".into()),
            port: Some(9000),
            log_level: Some("warn".into()),
            color: Some(true),
        };
        let env = PartialConfig {
            host: Some("env.local".into()),
            port: None,
            log_level: Some("debug".into()),
            color: Some(false),
        };
        let cli = PartialConfig {
            host: None,
            port: Some(7000),
            log_level: Some("trace".into()),
            color: None,
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&env), Some(&cli));

        assert_eq!(
            merged,
            Config {
                host: "env.local".into(),
                port: 7000,
                log_level: "trace".into(),
                color: false,
            }
        );
    }

    #[test]
    fn empty_cli_host_does_not_override_lower_layers() {
        let file = PartialConfig {
            host: Some("file.local".into()),
            port: None,
            log_level: None,
            color: None,
        };
        let env = PartialConfig {
            host: Some("env.local".into()),
            port: None,
            log_level: None,
            color: None,
        };
        let cli = PartialConfig {
            host: Some(String::new()),
            port: None,
            log_level: None,
            color: None,
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&env), Some(&cli));
        assert_eq!(merged.host, "env.local");
    }

    #[test]
    fn missing_layers_fall_back_to_defaults() {
        let merged = merge_config(&defaults(), None, None, None);
        assert_eq!(merged, defaults());
    }
}
