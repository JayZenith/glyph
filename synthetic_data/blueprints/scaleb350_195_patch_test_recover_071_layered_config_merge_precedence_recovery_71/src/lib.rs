#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub tls: bool,
    pub timeout_ms: u64,
    pub profile: String,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub tls: Option<bool>,
    pub timeout_ms: Option<u64>,
    pub profile: Option<String>,
}

impl Config {
    pub fn defaults() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            tls: false,
            timeout_ms: 3000,
            profile: "dev".to_string(),
        }
    }
}

pub fn merge_config(
    defaults: Config,
    file: PartialConfig,
    env: PartialConfig,
    cli: PartialConfig,
) -> Config {
    let mut out = defaults;

    apply_partial(&mut out, &cli);
    apply_partial(&mut out, &env);
    apply_partial(&mut out, &file);

    if out.timeout_ms == 0 {
        out.timeout_ms = 3000;
    }

    if out.profile == "prod" {
        out.tls = false;
    }

    out
}

fn apply_partial(base: &mut Config, layer: &PartialConfig) {
    if let Some(host) = &layer.host {
        base.host = host.clone();
    }
    if let Some(port) = layer.port {
        base.port = port;
    }
    if let Some(tls) = layer.tls {
        base.tls = tls;
    }
    if let Some(timeout_ms) = layer.timeout_ms {
        base.timeout_ms = timeout_ms;
    }
    if let Some(profile) = &layer.profile {
        base.profile = profile.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precedence_is_defaults_then_file_then_env_then_cli() {
        let defaults = Config::defaults();
        let file = PartialConfig {
            host: Some("file.internal".into()),
            port: Some(7000),
            tls: Some(false),
            timeout_ms: Some(4000),
            profile: Some("stage".into()),
        };
        let env = PartialConfig {
            host: Some("env.internal".into()),
            port: Some(9000),
            tls: Some(true),
            timeout_ms: None,
            profile: Some("prod".into()),
        };
        let cli = PartialConfig {
            host: None,
            port: Some(9500),
            tls: Some(false),
            timeout_ms: Some(1500),
            profile: None,
        };

        let merged = merge_config(defaults, file, env, cli);
        assert_eq!(
            merged,
            Config {
                host: "env.internal".into(),
                port: 9500,
                tls: false,
                timeout_ms: 1500,
                profile: "prod".into(),
            }
        );
    }

    #[test]
    fn zero_timeout_from_higher_precedence_layer_falls_back_to_default() {
        let defaults = Config::defaults();
        let file = PartialConfig {
            timeout_ms: Some(5000),
            ..PartialConfig::default()
        };
        let env = PartialConfig {
            timeout_ms: Some(0),
            ..PartialConfig::default()
        };
        let cli = PartialConfig::default();

        let merged = merge_config(defaults, file, env, cli);
        assert_eq!(merged.timeout_ms, 3000);
    }

    #[test]
    fn prod_profile_does_not_disable_explicit_tls_true_from_higher_precedence_layer() {
        let defaults = Config::defaults();
        let file = PartialConfig {
            profile: Some("prod".into()),
            tls: Some(false),
            ..PartialConfig::default()
        };
        let env = PartialConfig {
            tls: Some(true),
            ..PartialConfig::default()
        };
        let cli = PartialConfig::default();

        let merged = merge_config(defaults, file, env, cli);
        assert_eq!(merged.profile, "prod");
        assert!(merged.tls);
    }

    #[test]
    fn defaults_are_preserved_when_no_layer_sets_field() {
        let merged = merge_config(
            Config::defaults(),
            PartialConfig {
                host: Some("cfg.local".into()),
                ..PartialConfig::default()
            },
            PartialConfig::default(),
            PartialConfig::default(),
        );

        assert_eq!(merged.host, "cfg.local");
        assert_eq!(merged.port, 8080);
        assert!(!merged.tls);
        assert_eq!(merged.timeout_ms, 3000);
        assert_eq!(merged.profile, "dev");
    }
}
