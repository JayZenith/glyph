#[derive(Clone, Debug)]
struct Config {
    host: String,
    port: u16,
    mode: String,
}

#[derive(Clone, Debug)]
struct PartialConfig {
    host: Option<String>,
    port: Option<u16>,
    mode: Option<String>,
}

impl PartialConfig {
    fn empty() -> Self {
        Self {
            host: None,
            port: None,
            mode: None,
        }
    }
}

fn merge(base: Config, overlay: PartialConfig) -> Config {
    Config {
        host: overlay.host.unwrap_or(base.host),
        port: overlay.port.unwrap_or(base.port),
        mode: overlay.mode.unwrap_or(base.mode),
    }
}

fn main() {
    let defaults = Config {
        host: "localhost".to_string(),
        port: 8080,
        mode: "release".to_string(),
    };

    let file_cfg = PartialConfig {
        host: Some("file.example.com".to_string()),
        port: Some(9000),
        mode: None,
    };

    let env_cfg = PartialConfig {
        host: Some("env.example.com".to_string()),
        port: None,
        mode: Some("debug".to_string()),
    };

    let cli_cfg = PartialConfig {
        host: Some("".to_string()),
        port: Some(7000),
        mode: None,
    };

    let merged = merge(merge(merge(defaults, cli_cfg), env_cfg), file_cfg);

    println!(
        "{{\"host\":\"{}\",\"port\":{},\"mode\":\"{}\"}}",
        merged.host, merged.port, merged.mode
    );
}
