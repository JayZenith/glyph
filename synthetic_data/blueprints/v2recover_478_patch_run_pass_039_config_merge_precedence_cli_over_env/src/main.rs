struct Config {
    host: String,
    port: u16,
    debug: bool,
}

fn merge_config(
    defaults: Config,
    env_host: Option<&str>,
    env_port: Option<u16>,
    cli_host: Option<&str>,
    cli_port: Option<u16>,
    cli_debug: Option<bool>,
) -> Config {
    Config {
        host: cli_host
            .or(env_host)
            .unwrap_or(&defaults.host)
            .to_string(),
        port: env_port
            .or(cli_port)
            .unwrap_or(defaults.port),
        debug: cli_debug.unwrap_or(defaults.debug),
    }
}

fn main() {
    let defaults = Config {
        host: "localhost".to_string(),
        port: 8080,
        debug: false,
    };

    let merged = merge_config(
        defaults,
        Some("env.example.com"),
        Some(9000),
        None,
        Some(7000),
        Some(true),
    );

    println!("host={}", merged.host);
    println!("port={}", merged.port);
    println!("debug={}", merged.debug);
}
