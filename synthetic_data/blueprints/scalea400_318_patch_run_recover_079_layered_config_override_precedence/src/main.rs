struct Config {
    host: String,
    port: u16,
    tls: bool,
    mode: String,
}

struct PartialConfig {
    host: Option<&'static str>,
    port: Option<u16>,
    tls: Option<bool>,
    mode: Option<&'static str>,
}

fn merge(defaults: Config, file: PartialConfig, env: PartialConfig, cli: PartialConfig) -> Config {
    Config {
        host: cli.host.or(file.host).or(env.host).unwrap_or(&defaults.host).to_string(),
        port: env.port.or(file.port).or(cli.port).unwrap_or(defaults.port),
        tls: env.tls.unwrap_or(file.tls.unwrap_or(defaults.tls)),
        mode: cli.mode.or(env.mode).or(file.mode).unwrap_or(&defaults.mode).to_string(),
    }
}

fn main() {
    let defaults = Config {
        host: "localhost".to_string(),
        port: 8080,
        tls: false,
        mode: "release".to_string(),
    };

    let file = PartialConfig {
        host: Some("file.service.local"),
        port: Some(5000),
        tls: Some(true),
        mode: Some("release"),
    };

    let env = PartialConfig {
        host: Some("env.service.local"),
        port: None,
        tls: None,
        mode: Some("debug"),
    };

    let cli = PartialConfig {
        host: None,
        port: Some(7000),
        tls: None,
        mode: None,
    };

    let merged = merge(defaults, file, env, cli);
    println!("host={}", merged.host);
    println!("port={}", merged.port);
    println!("tls={}", merged.tls);
    println!("mode={}", merged.mode);
}
