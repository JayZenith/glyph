#[derive(Clone, Copy)]
struct Config {
    host: Option<&'static str>,
    port: Option<u16>,
    tls: Option<bool>,
    mode: Option<&'static str>,
    retries: Option<u8>,
}

impl Config {
    fn empty() -> Self {
        Self {
            host: None,
            port: None,
            tls: None,
            mode: None,
            retries: None,
        }
    }
}

fn merge(base: Config, overlay: Config) -> Config {
    Config {
        host: base.host.or(overlay.host),
        port: base.port.or(overlay.port),
        tls: base.tls.or(overlay.tls),
        mode: base.mode.or(overlay.mode),
        retries: base.retries.or(overlay.retries),
    }
}

fn main() {
    let defaults = Config {
        host: Some("localhost"),
        port: Some(8080),
        tls: Some(false),
        mode: Some("release"),
        retries: Some(3),
    };

    let file_cfg = Config {
        host: Some("db.example.com"),
        port: None,
        tls: Some(true),
        mode: None,
        retries: Some(5),
    };

    let env_cfg = Config {
        host: None,
        port: Some(7000),
        tls: None,
        mode: Some("debug"),
        retries: None,
    };

    let cfg = merge(merge(defaults, file_cfg), env_cfg);

    println!("host={}", cfg.host.unwrap_or(""));
    println!("port={}", cfg.port.unwrap_or(0));
    println!("tls={}", cfg.tls.unwrap_or(false));
    println!("mode={}", cfg.mode.unwrap_or(""));
    println!("retries={}", cfg.retries.unwrap_or(0));
}
