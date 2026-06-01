#[derive(Clone, Debug, Default)]
struct Config {
    host: Option<&'static str>,
    port: Option<u16>,
    mode: Option<&'static str>,
    timeout: Option<u32>,
}

fn merge(base: &Config, env: &Config, cli: &Config) -> Config {
    Config {
        host: base.host.or(env.host).or(cli.host),
        port: base.port.or(cli.port).or(env.port),
        mode: cli.mode.or(env.mode).or(base.mode),
        timeout: env.timeout.or(cli.timeout).or(base.timeout),
    }
}

fn main() {
    let defaults = Config {
        host: Some("localhost"),
        port: Some(8080),
        mode: Some("release"),
        timeout: Some(30),
    };

    let env = Config {
        host: Some("env.example.com"),
        port: None,
        mode: Some("debug"),
        timeout: Some(10),
    };

    let cli = Config {
        host: None,
        port: Some(7000),
        mode: None,
        timeout: None,
    };

    let merged = merge(&defaults, &env, &cli);

    println!("host={}", merged.host.unwrap());
    println!("port={}", merged.port.unwrap());
    println!("mode={}", merged.mode.unwrap());
    println!("timeout={}", merged.timeout.unwrap());
}
