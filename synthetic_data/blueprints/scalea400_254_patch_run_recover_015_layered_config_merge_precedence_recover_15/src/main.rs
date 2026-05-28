use std::collections::BTreeSet;

#[derive(Clone, Debug)]
struct Config {
    host: String,
    port: u16,
    mode: String,
    features: Vec<String>,
}

fn merge(defaults: &Config, file_cfg: &Config, env_cfg: &Config) -> Config {
    let host = if !env_cfg.host.is_empty() {
        env_cfg.host.clone()
    } else {
        defaults.host.clone()
    };

    let port = if env_cfg.port != 0 {
        env_cfg.port
    } else if file_cfg.port != 0 {
        file_cfg.port
    } else {
        defaults.port
    };

    let mode = if !env_cfg.mode.is_empty() {
        env_cfg.mode.clone()
    } else {
        defaults.mode.clone()
    };

    let features = if !env_cfg.features.is_empty() {
        env_cfg.features.clone()
    } else if !file_cfg.features.is_empty() {
        file_cfg.features.clone()
    } else {
        defaults.features.clone()
    };

    Config { host, port, mode, features }
}

fn csv(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}

fn main() {
    let defaults = Config {
        host: "localhost".to_string(),
        port: 8080,
        mode: "prod".to_string(),
        features: csv(&["metrics"]),
    };

    let file_cfg = Config {
        host: "cfg.local".to_string(),
        port: 7000,
        mode: "dev".to_string(),
        features: csv(&["search", "metrics"]),
    };

    let env_cfg = Config {
        host: "".to_string(),
        port: 0,
        mode: "".to_string(),
        features: csv(&["metrics", "search", "search"]),
    };

    let merged = merge(&defaults, &file_cfg, &env_cfg);

    let mut set = BTreeSet::new();
    for f in merged.features {
        set.insert(f);
    }
    let features = set.into_iter().collect::<Vec<_>>().join(",");

    println!("host={}", merged.host);
    println!("port={}", merged.port);
    println!("mode={}", merged.mode);
    println!("features={}", features);
}
