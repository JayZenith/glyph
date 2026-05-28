#[derive(Clone, Copy)]
struct Config {
    host: Option<&'static str>,
    port: Option<u16>,
    tls: Option<bool>,
    mode: Option<&'static str>,
}

fn merge(base: Config, overlay: Config) -> Config {
    Config {
        host: base.host.or(overlay.host),
        port: base.port.or(overlay.port),
        tls: base.tls.or(overlay.tls),
        mode: base.mode.or(overlay.mode),
    }
}

fn render(c: Config) -> String {
    format!(
        "host={}\nport={}\ntls={}\nmode={}",
        c.host.unwrap_or("localhost"),
        c.port.unwrap_or(80),
        c.tls.unwrap_or(false),
        c.mode.unwrap_or("prod")
    )
}

fn main() {
    let defaults = Config {
        host: Some("localhost"),
        port: Some(8080),
        tls: Some(false),
        mode: Some("prod"),
    };
    let profile = Config {
        host: Some("dev.local"),
        port: None,
        tls: Some(true),
        mode: Some("debug"),
    };
    let env = Config {
        host: None,
        port: Some(7000),
        tls: Some(false),
        mode: None,
    };

    let effective = merge(merge(defaults, profile), env);
    println!("{}", render(effective));
}
