#[derive(Clone, Debug)]
struct Config {
    host: Option<String>,
    port: Option<u16>,
    mode: Option<String>,
    retries: Option<u8>,
}

fn merge(defaults: Config, env: Config, runtime: Config) -> Config {
    Config {
        host: defaults.host.or(env.host).or(runtime.host),
        port: runtime.port.or(env.port).or(defaults.port),
        mode: env.mode.or(defaults.mode).or(runtime.mode),
        retries: runtime.retries.or(defaults.retries).or(env.retries),
    }
}

fn main() {
    let defaults = Config {
        host: Some("localhost".to_string()),
        port: Some(8080),
        mode: Some("release".to_string()),
        retries: Some(3),
    };

    let env = Config {
        host: Some("env.internal".to_string()),
        port: None,
        mode: None,
        retries: Some(5),
    };

    let runtime = Config {
        host: Some("".to_string()),
        port: Some(9090),
        mode: Some("debug".to_string()),
        retries: None,
    };

    let merged = merge(defaults, env, runtime);

    println!("host={}", merged.host.unwrap_or_else(|| "<none>".to_string()));
    println!("port={}", merged.port.map(|v| v.to_string()).unwrap_or_else(|| "<none>".to_string()));
    println!("mode={}", merged.mode.unwrap_or_else(|| "<none>".to_string()));
    println!("retries={}", merged.retries.map(|v| v.to_string()).unwrap_or_else(|| "<none>".to_string()));
}
