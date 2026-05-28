#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
            debug: false,
        }
    }
}

pub fn merge_config(
    user: Option<Config>,
    env: Option<Config>,
    override_cfg: Option<Config>,
) -> Config {
    let mut cfg = Config::default();

    if let Some(u) = user {
        cfg = u;
    }

    if let Some(o) = override_cfg {
        cfg = o;
    }

    if let Some(e) = env {
        cfg = e;
    }

    cfg
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(host: &str, port: u16, debug: bool) -> Config {
        Config {
            host: host.to_string(),
            port,
            debug,
        }
    }

    #[test]
    fn uses_defaults_when_nothing_is_set() {
        assert_eq!(merge_config(None, None, None), Config::default());
    }

    #[test]
    fn env_overrides_user() {
        let merged = merge_config(
            Some(cfg("user.local", 3000, false)),
            Some(cfg("env.local", 4000, true)),
            None,
        );
        assert_eq!(merged, cfg("env.local", 4000, true));
    }

    #[test]
    fn override_has_highest_precedence() {
        let merged = merge_config(
            Some(cfg("user.local", 3000, false)),
            Some(cfg("env.local", 4000, true)),
            Some(cfg("override.local", 5000, false)),
        );
        assert_eq!(merged, cfg("override.local", 5000, false));
    }
}
