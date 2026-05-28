#[derive(Clone, Debug)]
struct Config {
    host: Option<String>,
    port: Option<u16>,
    mode: Option<String>,
    retries: Option<u8>,
}

fn merge(base: &Config, env: &Config, cli: &Config) -> Config {
    Config {
        host: cli.host.clone().or_else(|| env.host.clone()).or_else(|| base.host.clone()),
        port: cli.port.or(env.port).or(base.port),
        mode: cli
            .mode
            .as_ref()
            .filter(|s| !s.is_empty())
            .cloned()
            .or_else(|| env.mode.clone())
            .or_else(|| base.mode.clone()),
        retries: cli.retries.or(env.retries).or(base.retries),
    }
}

fn main() {
    let defaults = Config {
        host: Some("localhost".to_string()),
        port: Some(8080),
        mode: Some("safe".to_string()),
        retries: Some(3),
    };

    let env_cfg = Config {
        host: Some("db.prod.local".to_string()),
        port: None,
        mode: Some("fast".to_string()),
        retries: Some(1),
    };

    let cli_cfg = Config {
        host: None,
        port: Some(9090),
        mode: Some(String::new()),
        retries: None,
    };

    let merged = merge(&defaults, &env_cfg, &cli_cfg);

    println!("host={}", merged.host.unwrap_or_default());
    println!("port={}", merged.port.unwrap_or_default());
    println!("mode={}", merged.mode.unwrap_or_default());
    println!("retries={}", merged.retries.unwrap_or_default());
}
