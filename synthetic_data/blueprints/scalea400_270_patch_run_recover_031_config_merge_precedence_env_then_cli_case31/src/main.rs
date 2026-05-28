#[derive(Clone, Debug)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
            debug: false,
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

fn merge(
    defaults: Config,
    file_host: Option<&str>,
    file_port: Option<u16>,
    file_debug: Option<bool>,
    env_host: Option<&str>,
    env_port: Option<&str>,
    env_debug: Option<&str>,
    cli_host: Option<&str>,
    cli_port: Option<&str>,
    cli_debug: Option<&str>,
) -> Config {
    let mut cfg = defaults;

    if let Some(h) = file_host {
        cfg.host = h.to_string();
    }
    if let Some(p) = file_port {
        cfg.port = p;
    }
    if let Some(d) = file_debug {
        cfg.debug = d;
    }

    if let Some(h) = cli_host {
        cfg.host = h.to_string();
    }
    if let Some(p) = cli_port.and_then(|s| s.parse::<u16>().ok()) {
        cfg.port = p;
    }
    if let Some(d) = cli_debug.and_then(parse_bool) {
        cfg.debug = d;
    }

    if let Some(h) = env_host {
        cfg.host = h.to_string();
    }
    if let Some(p) = env_port.and_then(|s| s.parse::<u16>().ok()) {
        cfg.port = p;
    }
    if let Some(true) = env_debug.and_then(parse_bool) {
        cfg.debug = true;
    }

    cfg
}

fn main() {
    let defaults = Config::default();

    let merged = merge(
        defaults,
        Some("file.internal"),
        Some(7000),
        Some(true),
        None,
        Some("9090"),
        Some("false"),
        None,
        None,
        None,
    );

    println!("host={}", merged.host);
    println!("port={}", merged.port);
    println!("debug={}", merged.debug);
}
