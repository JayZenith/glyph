#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub endpoint: String,
    pub retries: u8,
    pub timeout_ms: u64,
    pub verbose: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct PartialSettings {
    pub endpoint: Option<&'static str>,
    pub retries: Option<u8>,
    pub timeout_ms: Option<u64>,
    pub verbose: Option<bool>,
}

impl PartialSettings {
    pub const fn empty() -> Self {
        Self {
            endpoint: None,
            retries: None,
            timeout_ms: None,
            verbose: None,
        }
    }
}

fn overlay(dst: &mut Settings, src: PartialSettings) {
    if let Some(v) = src.endpoint {
        dst.endpoint = v.to_string();
    }
    if let Some(v) = src.retries {
        dst.retries = v;
    }
    if let Some(v) = src.timeout_ms {
        dst.timeout_ms = v;
    }
    if let Some(v) = src.verbose {
        dst.verbose = v;
    }
}

fn profile_defaults(profile: Option<&str>) -> PartialSettings {
    match profile {
        Some("dev") => PartialSettings {
            endpoint: Some("https://dev.local"),
            retries: Some(1),
            timeout_ms: Some(500),
            verbose: Some(true),
        },
        Some("prod") => PartialSettings {
            endpoint: Some("https://api.service"),
            retries: Some(5),
            timeout_ms: Some(3_000),
            verbose: Some(false),
        },
        _ => PartialSettings::empty(),
    }
}

pub fn merge_settings(
    base: PartialSettings,
    env: PartialSettings,
    profile: Option<&str>,
    cli: PartialSettings,
) -> Settings {
    let mut out = Settings {
        endpoint: "http://localhost".to_string(),
        retries: 2,
        timeout_ms: 1_000,
        verbose: false,
    };

    overlay(&mut out, base);
    overlay(&mut out, cli);
    overlay(&mut out, profile_defaults(profile));
    overlay(&mut out, env);

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precedence_is_base_then_profile_then_env_then_cli() {
        let base = PartialSettings {
            endpoint: Some("https://base.local"),
            retries: Some(2),
            timeout_ms: Some(800),
            verbose: Some(false),
        };
        let env = PartialSettings {
            endpoint: None,
            retries: Some(4),
            timeout_ms: None,
            verbose: Some(true),
        };
        let cli = PartialSettings {
            endpoint: Some("https://cli.local"),
            retries: None,
            timeout_ms: Some(150),
            verbose: None,
        };

        let got = merge_settings(base, env, Some("prod"), cli);
        assert_eq!(
            got,
            Settings {
                endpoint: "https://cli.local".to_string(),
                retries: 4,
                timeout_ms: 150,
                verbose: true,
            }
        );
    }

    #[test]
    fn unknown_profile_falls_back_to_default_values_only() {
        let got = merge_settings(
            PartialSettings::empty(),
            PartialSettings::empty(),
            Some("qa"),
            PartialSettings::empty(),
        );

        assert_eq!(
            got,
            Settings {
                endpoint: "http://localhost".to_string(),
                retries: 2,
                timeout_ms: 1_000,
                verbose: false,
            }
        );
    }

    #[test]
    fn empty_profile_name_is_treated_like_no_profile() {
        let got = merge_settings(
            PartialSettings::empty(),
            PartialSettings::empty(),
            Some(""),
            PartialSettings {
                endpoint: None,
                retries: Some(9),
                timeout_ms: None,
                verbose: None,
            },
        );

        assert_eq!(got.retries, 9);
        assert_eq!(got.endpoint, "http://localhost");
        assert!(!got.verbose);
    }

    #[test]
    fn cli_false_must_override_env_true_for_boolean_flags() {
        let got = merge_settings(
            PartialSettings::empty(),
            PartialSettings {
                endpoint: None,
                retries: None,
                timeout_ms: None,
                verbose: Some(true),
            },
            Some("dev"),
            PartialSettings {
                endpoint: None,
                retries: None,
                timeout_ms: None,
                verbose: Some(false),
            },
        );

        assert!(!got.verbose);
        assert_eq!(got.endpoint, "https://dev.local");
    }
}
