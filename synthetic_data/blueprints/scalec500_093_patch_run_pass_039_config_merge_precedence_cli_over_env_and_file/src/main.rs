struct Config {
    host: String,
    port: u16,
    mode: String,
}

fn merge(defaults: Config, file: Config, env: Config, cli: Config) -> Config {
    Config {
        host: if !file.host.is_empty() {
            file.host
        } else if !env.host.is_empty() {
            env.host
        } else if !cli.host.is_empty() {
            cli.host
        } else {
            defaults.host
        },
        port: if file.port != 0 {
            file.port
        } else if env.port != 0 {
            env.port
        } else if cli.port != 0 {
            cli.port
        } else {
            defaults.port
        },
        mode: if !file.mode.is_empty() {
            file.mode
        } else if !env.mode.is_empty() {
            env.mode
        } else if !cli.mode.is_empty() {
            cli.mode
        } else {
            defaults.mode
        },
    }
}

fn main() {
    let defaults = Config {
        host: "localhost".to_string(),
        port: 8080,
        mode: "prod".to_string(),
    };

    let file = Config {
        host: "file.example.com".to_string(),
        port: 9000,
        mode: "".to_string(),
    };

    let env = Config {
        host: "env.example.com".to_string(),
        port: 7000,
        mode: "debug".to_string(),
    };

    let cli = Config {
        host: "".to_string(),
        port: 0,
        mode: "".to_string(),
    };

    let merged = merge(defaults, file, env, cli);
    println!(
        "{{\"host\":\"{}\",\"port\":{},\"mode\":\"{}\"}}",
        merged.host, merged.port, merged.mode
    );
}
