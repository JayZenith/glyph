#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub endpoint: String,
    pub timeout_ms: u64,
    pub retries: u8,
    pub cache_enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PartialSettings {
    pub endpoint: Option<String>,
    pub timeout_ms: Option<u64>,
    pub retries: Option<u8>,
    pub cache_enabled: Option<bool>,
}

pub fn merge_settings(
    defaults: &Settings,
    file: Option<&PartialSettings>,
    env: Option<&PartialSettings>,
) -> Settings {
    let mut merged = defaults.clone();

    if let Some(layer) = env {
        apply_layer(&mut merged, layer);
    }
    if let Some(layer) = file {
        apply_layer(&mut merged, layer);
    }

    merged
}

fn apply_layer(base: &mut Settings, layer: &PartialSettings) {
    if let Some(endpoint) = &layer.endpoint {
        base.endpoint = endpoint.clone();
    }
    if let Some(timeout) = layer.timeout_ms {
        base.timeout_ms = timeout;
    }
    if let Some(retries) = layer.retries {
        base.retries = retries;
    }
    if let Some(cache_enabled) = layer.cache_enabled {
        base.cache_enabled = cache_enabled;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Settings {
        Settings {
            endpoint: "https://default.service".into(),
            timeout_ms: 500,
            retries: 3,
            cache_enabled: true,
        }
    }

    #[test]
    fn env_overrides_file_and_defaults() {
        let file = PartialSettings {
            endpoint: Some("https://file.service".into()),
            timeout_ms: Some(800),
            retries: Some(4),
            cache_enabled: Some(false),
        };
        let env = PartialSettings {
            endpoint: Some("https://env.service".into()),
            timeout_ms: Some(1200),
            retries: None,
            cache_enabled: Some(true),
        };

        let merged = merge_settings(&defaults(), Some(&file), Some(&env));

        assert_eq!(merged.endpoint, "https://env.service");
        assert_eq!(merged.timeout_ms, 1200);
        assert_eq!(merged.retries, 4);
        assert!(merged.cache_enabled);
    }

    #[test]
    fn zero_timeout_from_layer_is_ignored() {
        let file = PartialSettings {
            endpoint: None,
            timeout_ms: Some(0),
            retries: Some(1),
            cache_enabled: None,
        };

        let merged = merge_settings(&defaults(), Some(&file), None);

        assert_eq!(merged.timeout_ms, 500);
        assert_eq!(merged.retries, 1);
    }

    #[test]
    fn blank_endpoint_does_not_override() {
        let env = PartialSettings {
            endpoint: Some("   ".into()),
            timeout_ms: None,
            retries: None,
            cache_enabled: Some(false),
        };

        let merged = merge_settings(&defaults(), None, Some(&env));

        assert_eq!(merged.endpoint, "https://default.service");
        assert!(!merged.cache_enabled);
    }

    #[test]
    fn retries_are_capped_at_ten_after_merge() {
        let file = PartialSettings {
            endpoint: None,
            timeout_ms: Some(700),
            retries: Some(12),
            cache_enabled: None,
        };
        let env = PartialSettings {
            endpoint: None,
            timeout_ms: None,
            retries: Some(15),
            cache_enabled: None,
        };

        let merged = merge_settings(&defaults(), Some(&file), Some(&env));

        assert_eq!(merged.retries, 10);
        assert_eq!(merged.timeout_ms, 700);
    }
}
