use std::fmt;

#[derive(Clone, Debug)]
struct Config {
    host: String,
    port: u16,
    timeout: u32,
    verbose: bool,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "host={}\nport={}\ntimeout={}\nverbose={}",
            self.host, self.port, self.timeout, self.verbose
        )
    }
}

#[derive(Default)]
struct PartialConfig {
    host: Option<String>,
    port: Option<u16>,
    timeout: Option<u32>,
    verbose: Option<bool>,
}

fn merge(defaults: Config, env: PartialConfig, cli: PartialConfig) -> Config {
    Config {
        host: cli.host.or(env.host).unwrap_or(defaults.host),
        port: cli.port.or(env.port).unwrap_or(defaults.port),
        timeout: env.timeout.or(cli.timeout).unwrap_or(defaults.timeout),
        verbose: env.verbose.or(cli.verbose).unwrap_or(defaults.verbose),
    }
}

fn main() {
    let defaults = Config {
        host: "localhost".to_string(),
        port: 8080,
        timeout: 30,
        verbose: false,
    };

    let env = PartialConfig {
        host: Some("env.example.com".to_string()),
        port: None,
        timeout: Some(15),
        verbose: Some(false),
    };

    let cli = PartialConfig {
        host: Some("".to_string()),
        port: Some(9090),
        timeout: None,
        verbose: Some(true),
    };

    let merged = merge(defaults, env, cli);
    println!("{}", merged);
}
