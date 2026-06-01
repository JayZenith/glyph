#[derive(Clone, Debug)]
struct Config {
    mode: Option<String>,
    retries: Option<u32>,
    verbose: Option<bool>,
    path: Option<String>,
    tags: Vec<String>,
}

impl Config {
    fn empty() -> Self {
        Self {
            mode: None,
            retries: None,
            verbose: None,
            path: None,
            tags: Vec::new(),
        }
    }
}

fn parse_bool(s: &str) -> Option<bool> {
    match s {
        "true" | "1" | "yes" | "on" => Some(true),
        "false" | "0" | "no" | "off" => Some(false),
        _ => None,
    }
}

fn parse_retries(s: &str) -> Option<u32> {
    s.parse::<u32>().ok().filter(|v| *v <= 10)
}

fn merge(base: &mut Config, layer: &Config) {
    if base.mode.is_none() {
        base.mode = layer.mode.clone();
    }
    if base.retries.is_none() {
        base.retries = layer.retries;
    }
    if layer.verbose == Some(true) {
        base.verbose = Some(true);
    }
    if base.path.is_none() {
        base.path = layer.path.clone();
    }
    if !layer.tags.is_empty() {
        base.tags = layer.tags.clone();
    }
}

fn main() {
    let defaults = Config {
        mode: Some("safe".to_string()),
        retries: Some(2),
        verbose: Some(true),
        path: Some("/opt/app".to_string()),
        tags: vec!["base".to_string()],
    };

    let profile = Config {
        mode: Some("fast".to_string()),
        retries: Some(4),
        verbose: Some(false),
        path: None,
        tags: vec!["profile".to_string()],
    };

    let env_pairs = [
        ("APP_MODE", "safe"),
        ("APP_RETRIES", "20"),
        ("APP_VERBOSE", "false"),
        ("APP_PATH", "/srv/env"),
        ("APP_TAGS", "env"),
    ];

    let cli_pairs = [
        ("mode", "turbo"),
        ("retries", "4"),
        ("verbose", "false"),
        ("path", "/srv/cli"),
        ("tags", "cli"),
    ];

    let mut env_cfg = Config::empty();
    for (k, v) in env_pairs {
        match k {
            "APP_MODE" => env_cfg.mode = Some(v.to_string()),
            "APP_RETRIES" => env_cfg.retries = parse_retries(v),
            "APP_VERBOSE" => env_cfg.verbose = parse_bool(v),
            "APP_PATH" => env_cfg.path = Some(v.to_string()),
            "APP_TAGS" => env_cfg.tags = v.split(',').map(|s| s.to_string()).collect(),
            _ => {}
        }
    }

    let mut cli_cfg = Config::empty();
    for (k, v) in cli_pairs {
        match k {
            "mode" => cli_cfg.mode = Some(v.to_string()),
            "retries" => cli_cfg.retries = parse_retries(v),
            "verbose" => cli_cfg.verbose = parse_bool(v),
            "path" => cli_cfg.path = Some(v.to_string()),
            "tags" => cli_cfg.tags = v.split(',').map(|s| s.to_string()).collect(),
            _ => {}
        }
    }

    let mut effective = Config::empty();
    merge(&mut effective, &cli_cfg);
    merge(&mut effective, &env_cfg);
    merge(&mut effective, &profile);
    merge(&mut effective, &defaults);

    println!("mode={}", effective.mode.unwrap_or_else(|| "unset".to_string()));
    println!("retries={}", effective.retries.unwrap_or(0));
    println!("verbose={}", effective.verbose.unwrap_or(false));
    println!("path={}", effective.path.unwrap_or_else(|| "unset".to_string()));
    println!("tags={}", effective.tags.join(","));
}
