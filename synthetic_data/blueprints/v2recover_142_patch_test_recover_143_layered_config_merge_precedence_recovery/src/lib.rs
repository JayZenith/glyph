#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub use_tls: bool,
    pub mode: Mode,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Dev,
    Prod,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub use_tls: Option<bool>,
    pub mode: Option<Mode>,
    pub timeout_ms: Option<u64>,
}

pub fn resolve_config(defaults: &Config, file: &PartialConfig, env: &PartialConfig, force_prod: bool) -> Config {
    let mut cfg = defaults.clone();

    if let Some(v) = file.host.as_ref() {
        cfg.host = v.clone();
    }
    if let Some(v) = env.host.as_ref() {
        cfg.host = v.clone();
    }

    if let Some(v) = file.port {
        cfg.port = v;
    }
    if let Some(v) = env.port {
        cfg.port = v;
    }

    if let Some(v) = file.use_tls {
        cfg.use_tls = v;
    }
    if let Some(v) = env.use_tls {
        cfg.use_tls = v;
    }

    if let Some(v) = env.mode {
        cfg.mode = v;
    }
    if let Some(v) = file.mode {
        cfg.mode = v;
    }

    if let Some(v) = env.timeout_ms {
        cfg.timeout_ms = v;
    } else if let Some(v) = file.timeout_ms {
        cfg.timeout_ms = v;
    }

    if force_prod {
        cfg.mode = Mode::Prod;
    }

    cfg
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "localhost".into(),
            port: 8080,
            use_tls: false,
            mode: Mode::Dev,
            timeout_ms: 5000,
        }
    }

    #[test]
    fn env_wins_over_file_for_overlapping_fields() {
        let file = PartialConfig {
            host: Some("file.internal".into()),
            port: Some(7000),
            use_tls: Some(false),
            mode: Some(Mode::Dev),
            timeout_ms: Some(3000),
        };
        let env = PartialConfig {
            host: Some("env.internal".into()),
            port: Some(9000),
            use_tls: Some(true),
            mode: Some(Mode::Prod),
            timeout_ms: Some(1200),
        };

        let cfg = resolve_config(&defaults(), &file, &env, false);
        assert_eq!(cfg.host, "env.internal");
        assert_eq!(cfg.port, 9000);
        assert!(cfg.use_tls);
        assert_eq!(cfg.mode, Mode::Prod);
        assert_eq!(cfg.timeout_ms, 1200);
    }

    #[test]
    fn file_fills_missing_values_and_defaults_remain_for_unspecified() {
        let file = PartialConfig {
            host: Some("file.internal".into()),
            timeout_ms: Some(2500),
            ..PartialConfig::default()
        };
        let env = PartialConfig {
            use_tls: Some(true),
            ..PartialConfig::default()
        };

        let cfg = resolve_config(&defaults(), &file, &env, false);
        assert_eq!(cfg.host, "file.internal");
        assert_eq!(cfg.port, 8080);
        assert!(cfg.use_tls);
        assert_eq!(cfg.mode, Mode::Dev);
        assert_eq!(cfg.timeout_ms, 2500);
    }

    #[test]
    fn force_prod_overrides_inputs_and_enables_tls_and_min_timeout() {
        let file = PartialConfig {
            mode: Some(Mode::Dev),
            use_tls: Some(false),
            timeout_ms: Some(200),
            ..PartialConfig::default()
        };
        let env = PartialConfig {
            mode: Some(Mode::Dev),
            use_tls: Some(false),
            timeout_ms: Some(400),
            ..PartialConfig::default()
        };

        let cfg = resolve_config(&defaults(), &file, &env, true);
        assert_eq!(cfg.mode, Mode::Prod);
        assert!(cfg.use_tls);
        assert_eq!(cfg.timeout_ms, 1000);
    }

    #[test]
    fn prod_mode_from_any_source_enables_tls_by_default() {
        let file = PartialConfig {
            mode: Some(Mode::Prod),
            ..PartialConfig::default()
        };
        let env = PartialConfig::default();

        let cfg = resolve_config(&defaults(), &file, &env, false);
        assert_eq!(cfg.mode, Mode::Prod);
        assert!(cfg.use_tls);
    }

    #[test]
    fn explicit_tls_override_can_disable_prod_default_when_not_forced() {
        let file = PartialConfig {
            mode: Some(Mode::Prod),
            use_tls: Some(true),
            ..PartialConfig::default()
        };
        let env = PartialConfig {
            use_tls: Some(false),
            ..PartialConfig::default()
        };

        let cfg = resolve_config(&defaults(), &file, &env, false);
        assert_eq!(cfg.mode, Mode::Prod);
        assert!(!cfg.use_tls);
    }

    #[test]
    fn forced_prod_keeps_explicit_host_and_port_overrides() {
        let file = PartialConfig {
            host: Some("file.internal".into()),
            port: Some(7001),
            ..PartialConfig::default()
        };
        let env = PartialConfig {
            host: Some("env.internal".into()),
            port: Some(7443),
            ..PartialConfig::default()
        };

        let cfg = resolve_config(&defaults(), &file, &env, true);
        assert_eq!(cfg.mode, Mode::Prod);
        assert_eq!(cfg.host, "env.internal");
        assert_eq!(cfg.port, 7443);
    }
}
