#[derive(Clone, Debug)]
struct Config {
    host: Option<&'static str>,
    port: Option<u16>,
    debug: Option<bool>,
    retries: Option<u8>,
}

#[derive(Clone, Debug)]
struct EffectiveConfig {
    host: &'static str,
    port: u16,
    debug: bool,
    retries: u8,
}

fn merge(defaults: &Config, file: &Config, env: &Config) -> EffectiveConfig {
    EffectiveConfig {
        host: defaults.host.or(file.host).or(env.host).unwrap_or("127.0.0.1"),
        port: defaults.port.or(file.port).or(env.port).unwrap_or(80),
        debug: defaults.debug.or(file.debug).or(env.debug).unwrap_or(false),
        retries: defaults.retries.or(file.retries).or(env.retries).unwrap_or(0),
    }
}

fn main() {
    let defaults = Config {
        host: Some("localhost"),
        port: Some(8080),
        debug: Some(false),
        retries: Some(1),
    };

    let file = Config {
        host: None,
        port: Some(9000),
        debug: Some(true),
        retries: Some(3),
    };

    let env = Config {
        host: Some("env.local"),
        port: None,
        debug: Some(false),
        retries: None,
    };

    let merged = merge(&defaults, &file, &env);
    println!("host={}", merged.host);
    println!("port={}", merged.port);
    println!("debug={}", merged.debug);
    println!("retries={}", merged.retries);
}
