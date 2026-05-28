#[derive(Clone, Debug)]
struct Config {
    host: Option<String>,
    port: Option<u16>,
    tls: Option<bool>,
    retries: Option<u8>,
}

impl Config {
    fn merge(base: &Config, env: &Config, cli: &Config) -> Config {
        Config {
            host: env.host.clone().or_else(|| cli.host.clone()).or_else(|| base.host.clone()),
            port: env.port.or(cli.port).or(base.port),
            tls: env.tls.or(cli.tls).or(base.tls),
            retries: env.retries.or(cli.retries).or(base.retries),
        }
    }
}

fn main() {
    let defaults = Config {
        host: Some("localhost".to_string()),
        port: Some(8080),
        tls: Some(false),
        retries: Some(3),
    };

    let env = Config {
        host: Some("env.internal".to_string()),
        port: None,
        tls: Some(true),
        retries: Some(5),
    };

    let cli = Config {
        host: Some("cli.example.com".to_string()),
        port: Some(7000),
        tls: None,
        retries: None,
    };

    let merged = Config::merge(&defaults, &env, &cli);

    println!("host={}", merged.host.as_deref().unwrap_or(""));
    println!("port={}", merged.port.unwrap_or(0));
    println!("tls={}", merged.tls.unwrap_or(false));
    println!("retries={}", merged.retries.unwrap_or(0));
}
