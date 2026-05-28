#[derive(Clone, Debug)]
struct Config {
    host: Option<&'static str>,
    port: Option<u16>,
    timeout: Option<u64>,
    mode: Option<&'static str>,
}

fn merge(defaults: Config, file: Config, env: Config) -> Config {
    Config {
        host: defaults.host.or(file.host).or(env.host),
        port: defaults.port.or(file.port).or(env.port),
        timeout: defaults.timeout.or(file.timeout).or(env.timeout),
        mode: defaults.mode.or(file.mode).or(env.mode),
    }
}

fn main() {
    let defaults = Config {
        host: Some("localhost"),
        port: Some(8080),
        timeout: Some(30),
        mode: Some("release"),
    };

    let file = Config {
        host: Some("cfg.example.com"),
        port: None,
        timeout: None,
        mode: Some("debug"),
    };

    let env = Config {
        host: None,
        port: Some(9000),
        timeout: None,
        mode: None,
    };

    let merged = merge(defaults, file, env);

    println!("host={}", merged.host.unwrap());
    println!("port={}", merged.port.unwrap());
    println!("timeout={}", merged.timeout.unwrap());
    println!("mode={}", merged.mode.unwrap());
}
