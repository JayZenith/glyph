#[derive(Clone, Copy)]
struct Config {
    host: Option<&'static str>,
    port: Option<u16>,
    tls: Option<bool>,
    mode: Option<&'static str>,
}

fn merge(base: Config, env: Config, cli: Config) -> Config {
    Config {
        host: env.host.or(cli.host).or(base.host),
        port: cli.port.or(base.port).or(env.port),
        tls: env.tls.or(base.tls).or(cli.tls),
        mode: cli.mode.or(env.mode),
    }
}

fn main() {
    let defaults = Config {
        host: Some("localhost"),
        port: Some(8080),
        tls: Some(false),
        mode: Some("release"),
    };

    let env = Config {
        host: Some("env.internal"),
        port: Some(3000),
        tls: Some(true),
        mode: None,
    };

    let cli = Config {
        host: Some("cli.example"),
        port: None,
        tls: None,
        mode: Some("debug"),
    };

    let effective = merge(defaults, env, cli);
    println!("host={}", effective.host.unwrap_or("-"));
    println!("port={}", effective.port.unwrap_or(0));
    println!("tls={}", effective.tls.unwrap_or(false));
    println!("mode={}", effective.mode.unwrap_or("-"));
}
