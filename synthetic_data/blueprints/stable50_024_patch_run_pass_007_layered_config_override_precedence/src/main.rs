#[derive(Clone, Debug)]
struct Config {
    host: Option<&'static str>,
    port: Option<u16>,
    tls: Option<bool>,
    retries: Option<u8>,
}

fn merge(base: Config, overlay: Config) -> Config {
    Config {
        host: base.host.or(overlay.host),
        port: base.port.or(overlay.port),
        tls: base.tls.or(overlay.tls),
        retries: base.retries.or(overlay.retries),
    }
}

fn main() {
    let defaults = Config {
        host: Some("localhost"),
        port: Some(8080),
        tls: Some(false),
        retries: Some(3),
    };

    let env_cfg = Config {
        host: Some("env.local"),
        port: None,
        tls: Some(true),
        retries: Some(1),
    };

    let cli_cfg = Config {
        host: None,
        port: Some(9090),
        tls: None,
        retries: None,
    };

    let merged = merge(merge(defaults, env_cfg), cli_cfg);

    println!(
        "{{\"host\":\"{}\",\"port\":{},\"tls\":{},\"retries\":{}}}",
        merged.host.unwrap(),
        merged.port.unwrap(),
        merged.tls.unwrap(),
        merged.retries.unwrap()
    );
}
