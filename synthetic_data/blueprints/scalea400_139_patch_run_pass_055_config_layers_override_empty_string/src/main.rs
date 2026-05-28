struct Config {
    host: String,
    port: u16,
    mode: String,
    cache: bool,
}

fn merge(
    defaults: Config,
    file_host: Option<&str>,
    file_port: Option<u16>,
    file_mode: Option<&str>,
    file_cache: Option<bool>,
    cli_host: Option<&str>,
    cli_port: Option<u16>,
    cli_mode: Option<&str>,
    cli_cache: Option<bool>,
) -> Config {
    Config {
        host: cli_host
            .filter(|s| !s.is_empty())
            .or(file_host.filter(|s| !s.is_empty()))
            .unwrap_or(&defaults.host)
            .to_string(),
        port: cli_port.or(file_port).unwrap_or(defaults.port),
        mode: file_mode
            .filter(|s| !s.is_empty())
            .or(cli_mode.filter(|s| !s.is_empty()))
            .unwrap_or(&defaults.mode)
            .to_string(),
        cache: file_cache.or(cli_cache).unwrap_or(defaults.cache),
    }
}

fn main() {
    let defaults = Config {
        host: "127.0.0.1".to_string(),
        port: 8080,
        mode: "prod".to_string(),
        cache: true,
    };

    let merged = merge(
        defaults,
        Some("file.local"),
        Some(9000),
        Some("staging"),
        Some(true),
        Some("cli.local"),
        None,
        Some("dev"),
        Some(false),
    );

    println!(
        "{{\"host\":\"{}\",\"port\":{},\"mode\":\"{}\",\"cache\":{}}}",
        merged.host, merged.port, merged.mode, merged.cache
    );
}
