#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Defaults {
    pub host: &'static str,
    pub port: u16,
    pub retries: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FileConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub retries: Option<u8>,
    pub lock_port: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EnvConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub retries: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedConfig {
    pub host: String,
    pub port: u16,
    pub retries: u8,
}

pub fn merge_config(defaults: &Defaults, file: &FileConfig, env: &EnvConfig) -> ResolvedConfig {
    let mut cfg = ResolvedConfig {
        host: defaults.host.to_string(),
        port: defaults.port,
        retries: defaults.retries,
    };

    if let Some(host) = &file.host {
        cfg.host = host.clone();
    }
    if let Some(port) = file.port {
        cfg.port = port;
    }
    if let Some(retries) = file.retries {
        cfg.retries = retries;
    }

    if let Some(host) = &env.host {
        cfg.host = host.clone();
    }
    if let Some(port) = env.port {
        cfg.port = port;
    }
    if let Some(retries) = env.retries {
        cfg.retries = retries;
    }

    cfg
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Defaults {
        Defaults {
            host: "localhost",
            port: 8080,
            retries: 3,
        }
    }

    #[test]
    fn env_overrides_unlocked_file_host_and_port() {
        let file = FileConfig {
            host: Some("file.internal".into()),
            port: Some(9000),
            retries: None,
            lock_port: false,
        };
        let env = EnvConfig {
            host: Some("env.internal".into()),
            port: Some(7000),
            retries: None,
        };

        let merged = merge_config(&defaults(), &file, &env);
        assert_eq!(
            merged,
            ResolvedConfig {
                host: "env.internal".into(),
                port: 7000,
                retries: 3,
            }
        );
    }

    #[test]
    fn locked_file_port_wins_over_env() {
        let file = FileConfig {
            host: None,
            port: Some(9000),
            retries: None,
            lock_port: true,
        };
        let env = EnvConfig {
            host: None,
            port: Some(7000),
            retries: None,
        };

        let merged = merge_config(&defaults(), &file, &env);
        assert_eq!(merged.port, 9000);
    }

    #[test]
    fn env_retries_only_fill_missing_value() {
        let file = FileConfig {
            host: None,
            port: None,
            retries: Some(5),
            lock_port: false,
        };
        let env = EnvConfig {
            host: None,
            port: None,
            retries: Some(8),
        };

        let merged = merge_config(&defaults(), &file, &env);
        assert_eq!(merged.retries, 5);
    }

    #[test]
    fn env_retries_can_fill_when_file_missing() {
        let file = FileConfig {
            host: None,
            port: None,
            retries: None,
            lock_port: false,
        };
        let env = EnvConfig {
            host: None,
            port: None,
            retries: Some(8),
        };

        let merged = merge_config(&defaults(), &file, &env);
        assert_eq!(merged.retries, 8);
    }
}
