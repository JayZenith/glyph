struct Config {
    host: &'static str,
    port: u16,
    mode: &'static str,
}

#[derive(Clone, Copy)]
struct PartialConfig {
    host: Option<&'static str>,
    port: Option<u16>,
    mode: Option<&'static str>,
}

fn merge(defaults: Config, file: PartialConfig, env: PartialConfig, cli: PartialConfig) -> Config {
    Config {
        host: file.host.or(env.host).or(cli.host).unwrap_or(defaults.host),
        port: file.port.or(env.port).or(cli.port).unwrap_or(defaults.port),
        mode: file.mode.or(env.mode).or(cli.mode).unwrap_or(defaults.mode),
    }
}

fn main() {
    let defaults = Config {
        host: "localhost",
        port: 8080,
        mode: "release",
    };

    let file = PartialConfig {
        host: Some("file.example.com"),
        port: Some(3000),
        mode: None,
    };

    let env = PartialConfig {
        host: Some("env.example.com"),
        port: None,
        mode: Some("debug"),
    };

    let cli = PartialConfig {
        host: None,
        port: Some(9090),
        mode: None,
    };

    let merged = merge(defaults, file, env, cli);
    println!(
        "{{\"host\":\"{}\",\"port\":{},\"mode\":\"{}\"}}",
        merged.host, merged.port, merged.mode
    );
}
