#[derive(Clone, Debug)]
struct Config {
    host: Option<&'static str>,
    port: Option<u16>,
    timeout: Option<u16>,
    secure: Option<bool>,
}

fn merge(base: Config, layer: Config) -> Config {
    Config {
        host: base.host.or(layer.host),
        port: base.port.or(layer.port),
        timeout: base.timeout.or(layer.timeout),
        secure: base.secure.or(layer.secure),
    }
}

fn main() {
    let defaults = Config {
        host: Some("localhost"),
        port: Some(8080),
        timeout: Some(30),
        secure: Some(false),
    };

    let env_cfg = Config {
        host: None,
        port: Some(9000),
        timeout: Some(20),
        secure: Some(true),
    };

    let file_cfg = Config {
        host: Some("file.internal"),
        port: None,
        timeout: Some(45),
        secure: None,
    };

    let cli_cfg = Config {
        host: Some("cli.example.com"),
        port: Some(7000),
        timeout: None,
        secure: None,
    };

    let merged = merge(merge(merge(defaults, env_cfg), file_cfg), cli_cfg);

    println!("host={}", merged.host.unwrap());
    println!("port={}", merged.port.unwrap());
    println!("timeout={}", merged.timeout.unwrap());
    println!("secure={}", merged.secure.unwrap());
}
