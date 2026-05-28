#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub feature: Option<bool>,
}

impl Config {
    pub fn new(host: &str, port: u16, feature: Option<bool>) -> Self {
        Self {
            host: host.to_string(),
            port,
            feature,
        }
    }
}

pub fn merge_config(defaults: &Config, file: Option<&Config>, env: Option<&Config>) -> Config {
    let mut merged = defaults.clone();

    if let Some(file_cfg) = file {
        merged.host = file_cfg.host.clone();
        merged.port = file_cfg.port;
        merged.feature = file_cfg.feature;
    }

    if let Some(env_cfg) = env {
        merged.host = env_cfg.host.clone();
        merged.port = env_cfg.port;
        merged.feature = env_cfg.feature.or(merged.feature);
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_overrides_file_for_basic_fields() {
        let defaults = Config::new("127.0.0.1", 8080, Some(false));
        let file = Config::new("file.local", 9000, Some(true));
        let env = Config::new("env.local", 7000, None);

        let merged = merge_config(&defaults, Some(&file), Some(&env));

        assert_eq!(merged.host, "env.local");
        assert_eq!(merged.port, 7000);
        assert_eq!(merged.feature, Some(true));
    }

    #[test]
    fn missing_env_feature_keeps_file_feature() {
        let defaults = Config::new("127.0.0.1", 8080, Some(false));
        let file = Config::new("file.local", 9000, Some(true));
        let env = Config::new("env.local", 7000, None);

        let merged = merge_config(&defaults, Some(&file), Some(&env));

        assert_eq!(merged.feature, Some(true));
    }

    #[test]
    fn explicit_file_disable_wins_over_env_enable() {
        let defaults = Config::new("127.0.0.1", 8080, None);
        let file = Config::new("file.local", 9000, Some(false));
        let env = Config::new("env.local", 7000, Some(true));

        let merged = merge_config(&defaults, Some(&file), Some(&env));

        assert_eq!(merged.feature, Some(false));
    }

    #[test]
    fn env_can_set_feature_when_file_is_unspecified() {
        let defaults = Config::new("127.0.0.1", 8080, None);
        let file = Config::new("file.local", 9000, None);
        let env = Config::new("env.local", 7000, Some(true));

        let merged = merge_config(&defaults, Some(&file), Some(&env));

        assert_eq!(merged.feature, Some(true));
    }

    #[test]
    fn defaults_are_used_when_no_other_layers_exist() {
        let defaults = Config::new("127.0.0.1", 8080, Some(false));

        let merged = merge_config(&defaults, None, None);

        assert_eq!(merged, defaults);
    }
}
