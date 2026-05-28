use std::fmt::Write;

#[derive(Clone, Debug)]
struct Config {
    host: Option<&'static str>,
    port: Option<u16>,
    timeout: Option<u16>,
    verbose: Option<bool>,
}

fn merge(defaults: Config, env: Config, cli: Config) -> Config {
    Config {
        host: defaults.host.or(env.host).or(cli.host),
        port: defaults.port.or(env.port).or(cli.port),
        timeout: defaults.timeout.or(env.timeout).or(cli.timeout),
        verbose: defaults.verbose.or(env.verbose).or(cli.verbose),
    }
}

fn render(cfg: &Config) -> String {
    let mut out = String::new();
    writeln!(&mut out, "host={}", cfg.host.unwrap()).unwrap();
    writeln!(&mut out, "port={}", cfg.port.unwrap()).unwrap();
    writeln!(&mut out, "timeout={}", cfg.timeout.unwrap()).unwrap();
    write!(&mut out, "verbose={}", cfg.verbose.unwrap()).unwrap();
    out
}

fn main() {
    let defaults = Config {
        host: Some("localhost"),
        port: Some(8080),
        timeout: Some(30),
        verbose: Some(false),
    };

    let env = Config {
        host: Some("env.example.com"),
        port: None,
        timeout: None,
        verbose: Some(true),
    };

    let cli = Config {
        host: None,
        port: Some(9000),
        timeout: None,
        verbose: None,
    };

    let merged = merge(defaults, env, cli);
    print!("{}", render(&merged));
}
