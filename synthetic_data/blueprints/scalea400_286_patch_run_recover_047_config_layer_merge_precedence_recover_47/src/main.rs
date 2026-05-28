#[derive(Clone, Copy, Debug)]
struct Config {
    host: Option<&'static str>,
    port: Option<u16>,
    mode: Option<&'static str>,
    retries: Option<u8>,
}

fn merge(base: Config, overlay: Config) -> Config {
    Config {
        host: overlay.host.or(base.host),
        port: overlay.port.or(base.port),
        mode: overlay.mode.or(base.mode),
        retries: base.retries.or(overlay.retries),
    }
}

fn main() {
    let defaults = Config {
        host: Some("localhost"),
        port: Some(8080),
        mode: Some("prod"),
        retries: Some(3),
    };

    let file = Config {
        host: Some("file.example"),
        port: None,
        mode: None,
        retries: Some(5),
    };

    let env = Config {
        host: None,
        port: Some(9000),
        mode: Some("debug"),
        retries: None,
    };

    let cli = Config {
        host: Some(""),
        port: None,
        mode: None,
        retries: None,
    };

    let merged = merge(merge(merge(defaults, file), env), cli);

    println!(
        "host={};port={};mode={};retries={}",
        merged.host.unwrap_or("<none>"),
        merged.port.unwrap_or(0),
        merged.mode.unwrap_or("<none>"),
        merged.retries.unwrap_or(0)
    );
}
