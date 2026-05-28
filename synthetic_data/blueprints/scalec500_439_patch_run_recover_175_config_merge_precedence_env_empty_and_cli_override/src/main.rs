#[derive(Clone, Debug)]
struct Config {
    service: String,
    endpoint: String,
    retries: u32,
    timeout_ms: u32,
    dry_run: bool,
    tags: Vec<String>,
}

impl Config {
    fn defaults() -> Self {
        Self {
            service: "ingest".to_string(),
            endpoint: "https://default.example/api".to_string(),
            retries: 2,
            timeout_ms: 1000,
            dry_run: false,
            tags: vec!["base".to_string()],
        }
    }
}

#[derive(Clone, Debug, Default)]
struct PartialConfig {
    service: Option<String>,
    endpoint: Option<String>,
    retries: Option<u32>,
    timeout_ms: Option<u32>,
    dry_run: Option<bool>,
    tags: Option<Vec<String>>,
}

fn merge(mut base: Config, layer: &PartialConfig) -> Config {
    if let Some(v) = &layer.service {
        base.service = v.clone();
    }
    if let Some(v) = &layer.endpoint {
        base.endpoint = v.clone();
    }
    if let Some(v) = layer.retries {
        base.retries = v;
    }
    if let Some(v) = layer.timeout_ms {
        base.timeout_ms = v;
    }
    if let Some(v) = layer.dry_run {
        base.dry_run = v;
    }
    if let Some(v) = &layer.tags {
        base.tags.extend(v.clone());
    }
    base
}

fn render(cfg: &Config) -> String {
    format!(
        "service={}\nendpoint={}\nretries={}\ntimeout_ms={}\ndry_run={}\ntags={}",
        cfg.service,
        cfg.endpoint,
        cfg.retries,
        cfg.timeout_ms,
        cfg.dry_run,
        cfg.tags.join(",")
    )
}

fn main() {
    let file_cfg = PartialConfig {
        endpoint: Some("https://file.example/api".to_string()),
        retries: Some(5),
        timeout_ms: Some(2500),
        tags: Some(vec!["file".to_string()]),
        ..PartialConfig::default()
    };

    let env_cfg = PartialConfig {
        endpoint: Some("".to_string()),
        retries: Some(7),
        dry_run: Some(true),
        tags: Some(vec!["env".to_string()]),
        ..PartialConfig::default()
    };

    let cli_cfg = PartialConfig {
        endpoint: Some("https://cli.example/api".to_string()),
        tags: Some(vec!["cli".to_string()]),
        ..PartialConfig::default()
    };

    let mut cfg = Config::defaults();
    cfg = merge(cfg, &file_cfg);
    cfg = merge(cfg, &cli_cfg);
    cfg = merge(cfg, &env_cfg);

    println!("{}", render(&cfg));
}
