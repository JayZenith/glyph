#[derive(Clone, Copy)]
struct Config {
    host: Option<&'static str>,
    port: Option<u16>,
    verbose: Option<bool>,
}

fn merge(low: Config, high: Config) -> Config {
    Config {
        host: low.host.or(high.host),
        port: low.port.or(high.port),
        verbose: low.verbose.or(high.verbose),
    }
}

fn main() {
    let defaults = Config {
        host: Some("localhost"),
        port: Some(8080),
        verbose: Some(false),
    };

    let file_cfg = Config {
        host: Some("file.local"),
        port: Some(5000),
        verbose: None,
    };

    let env_cfg = Config {
        host: Some("env.local"),
        port: None,
        verbose: Some(true),
    };

    let cli_cfg = Config {
        host: None,
        port: Some(7000),
        verbose: Some(true),
    };

    let mut merged = merge(defaults, file_cfg);
    merged = merge(merged, env_cfg);

    if cli_cfg.verbose == Some(true) {
        merged = cli_cfg;
    } else {
        merged = merge(merged, cli_cfg);
    }

    println!("host={}", merged.host.unwrap());
    println!("port={}", merged.port.unwrap());
    println!("verbose={}", merged.verbose.unwrap());
}
