#[derive(Clone, Copy)]
struct Config {
    host: Option<&'static str>,
    port: Option<u16>,
    tls: Option<bool>,
    mode: Option<&'static str>,
}

fn merge(defaults: Config, file: Config, env: Config, cli: Config) -> Config {
    Config {
        host: defaults.host.or(file.host).or(env.host).or(cli.host),
        port: defaults.port.or(file.port).or(env.port).or(cli.port),
        tls: defaults.tls.or(file.tls).or(env.tls).or(cli.tls),
        mode: defaults.mode.or(file.mode).or(env.mode).or(cli.mode),
    }
}

fn main() {
    let defaults = Config {
        host: Some("localhost"),
        port: Some(8080),
        tls: Some(false),
        mode: Some("dev"),
    };

    let file = Config {
        host: Some("file.local"),
        port: Some(9000),
        tls: None,
        mode: Some("staging"),
    };

    let env = Config {
        host: None,
        port: Some(7000),
        tls: Some(true),
        mode: None,
    };

    let cli = Config {
        host: Some("cli.local"),
        port: None,
        tls: None,
        mode: Some("prod"),
    };

    let effective = merge(defaults, file, env, cli);

    println!("host={}", effective.host.unwrap());
    println!("port={}", effective.port.unwrap());
    println!("tls={}", effective.tls.unwrap());
    println!("mode={}", effective.mode.unwrap());
}
