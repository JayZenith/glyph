#[derive(Clone, Debug)]
struct Config {
    host: String,
    port: u16,
    mode: String,
    name: String,
}

#[derive(Clone, Debug, Default)]
struct PartialConfig {
    host: Option<String>,
    port: Option<u16>,
    mode: Option<String>,
    name: Option<String>,
}

fn merge(defaults: Config, env: PartialConfig, cli: PartialConfig) -> Config {
    Config {
        host: defaults.host,
        port: defaults.port,
        mode: defaults.mode,
        name: defaults.name,
    }
}

fn main() {
    let defaults = Config {
        host: "localhost".to_string(),
        port: 8080,
        mode: "release".to_string(),
        name: "app".to_string(),
    };

    let env = PartialConfig {
        host: Some("env.local".to_string()),
        port: None,
        mode: Some("debug".to_string()),
        name: Some("envsvc".to_string()),
    };

    let cli = PartialConfig {
        host: None,
        port: Some(9000),
        mode: None,
        name: Some(String::new()),
    };

    let cfg = merge(defaults, env, cli);
    println!("host={}\nport={}\nmode={}\nname={}", cfg.host, cfg.port, cfg.mode, cfg.name);
}
