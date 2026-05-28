use std::fmt::Write;

#[derive(Clone, Debug)]
struct Config {
    host: String,
    port: u16,
    mode: String,
    verbose: bool,
}

#[derive(Default)]
struct PartialConfig {
    host: Option<String>,
    port: Option<u16>,
    mode: Option<String>,
    verbose: Option<bool>,
}

fn defaults() -> Config {
    Config {
        host: "localhost".to_string(),
        port: 8080,
        mode: "safe".to_string(),
        verbose: false,
    }
}

fn env_config() -> PartialConfig {
    PartialConfig {
        host: Some("env.internal".to_string()),
        port: Some(9090),
        mode: Some("debug".to_string()),
        verbose: Some(true),
    }
}

fn cli_config() -> PartialConfig {
    PartialConfig {
        host: None,
        port: Some(7000),
        mode: None,
        verbose: None,
    }
}

fn merge(defaults: Config, env: PartialConfig, cli: PartialConfig) -> Config {
    let host = cli.host.or(env.host).unwrap_or(defaults.host);
    let port = env.port.or(cli.port).unwrap_or(defaults.port);
    let mode = env.mode.or(cli.mode).unwrap_or(defaults.mode);
    let verbose = cli.verbose.or(env.verbose).unwrap_or(defaults.verbose);

    Config {
        host,
        port,
        mode,
        verbose,
    }
}

fn render(cfg: &Config) -> String {
    let mut out = String::new();
    write!(
        &mut out,
        "{{\"host\":\"{}\",\"port\":{},\"mode\":\"{}\",\"verbose\":{}}}",
        cfg.host, cfg.port, cfg.mode, cfg.verbose
    )
    .unwrap();
    out
}

fn main() {
    let cfg = merge(defaults(), env_config(), cli_config());
    print!("{}", render(&cfg));
}
