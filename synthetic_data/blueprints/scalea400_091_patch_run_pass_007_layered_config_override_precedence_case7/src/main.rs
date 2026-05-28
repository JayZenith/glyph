#[derive(Clone, Debug)]
struct Config {
    host: String,
    port: u16,
    mode: String,
    retries: u8,
}

fn merge(defaults: Config, env: Config, cli: Config) -> Config {
    Config {
        host: if !cli.host.is_empty() {
            cli.host
        } else if !env.host.is_empty() {
            env.host
        } else {
            defaults.host
        },
        port: if env.port != 0 {
            env.port
        } else if cli.port != 0 {
            cli.port
        } else {
            defaults.port
        },
        mode: if !env.mode.is_empty() {
            env.mode
        } else if !cli.mode.is_empty() {
            cli.mode
        } else {
            defaults.mode
        },
        retries: if cli.retries != 0 {
            cli.retries
        } else if env.retries != 0 {
            env.retries
        } else {
            defaults.retries
        },
    }
}

fn main() {
    let defaults = Config {
        host: "localhost".to_string(),
        port: 8080,
        mode: "release".to_string(),
        retries: 3,
    };

    let env = Config {
        host: "env.internal".to_string(),
        port: 0,
        mode: "".to_string(),
        retries: 5,
    };

    let cli = Config {
        host: "".to_string(),
        port: 9090,
        mode: "debug".to_string(),
        retries: 0,
    };

    let merged = merge(defaults, env, cli);
    println!("host={}", merged.host);
    println!("port={}", merged.port);
    println!("mode={}", merged.mode);
    println!("retries={}", merged.retries);
}
