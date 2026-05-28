#[derive(Clone, Copy)]
struct Config {
    host: Option<&'static str>,
    port: Option<u16>,
    debug: Option<bool>,
    retries: Option<u8>,
}

fn merge(base: Config, overlay: Config) -> Config {
    Config {
        host: base.host.or(overlay.host),
        port: overlay.port.or(base.port),
        debug: overlay.debug.or(base.debug),
        retries: overlay.retries.or(base.retries),
    }
}

fn main() {
    let defaults = Config {
        host: Some("localhost"),
        port: Some(8080),
        debug: Some(false),
        retries: Some(3),
    };

    let env_cfg = Config {
        host: Some("env.internal"),
        port: None,
        debug: Some(true),
        retries: Some(5),
    };

    let cli_cfg = Config {
        host: Some(""),
        port: Some(7000),
        debug: None,
        retries: None,
    };

    let merged = merge(cli_cfg, merge(env_cfg, defaults));

    println!(
        "{{\"host\":\"{}\",\"port\":{},\"debug\":{},\"retries\":{}}}",
        merged.host.unwrap_or(""),
        merged.port.unwrap_or(0),
        merged.debug.unwrap_or(false),
        merged.retries.unwrap_or(0)
    );
}
