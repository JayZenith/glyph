#[derive(Clone, Debug)]
struct Config {
    host: Option<&'static str>,
    port: Option<u16>,
    timeout: Option<u16>,
    mode: Option<&'static str>,
}

impl Config {
    fn merged(defaults: Config, env: Config, cli: Config) -> Config {
        Config {
            host: defaults.host.or(env.host).or(cli.host),
            port: defaults.port.or(env.port).or(cli.port),
            timeout: defaults.timeout.or(env.timeout).or(cli.timeout),
            mode: defaults.mode.or(env.mode).or(cli.mode),
        }
    }
}

fn main() {
    let defaults = Config {
        host: Some("localhost"),
        port: Some(8080),
        timeout: Some(30),
        mode: Some("release"),
    };

    let env = Config {
        host: Some("env.example.com"),
        port: None,
        timeout: Some(60),
        mode: Some("debug"),
    };

    let cli = Config {
        host: None,
        port: Some(9090),
        timeout: None,
        mode: None,
    };

    let merged = Config::merged(defaults, env, cli);

    println!("host={}", merged.host.unwrap());
    println!("port={}", merged.port.unwrap());
    println!("timeout={}", merged.timeout.unwrap());
    println!("mode={}", merged.mode.unwrap());
}
