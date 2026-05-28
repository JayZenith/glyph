#[derive(Clone, Debug)]
struct Config {
    host: Option<String>,
    port: Option<u16>,
    debug: Option<bool>,
    workers: Option<u8>,
}

fn merge(defaults: Config, file: Config, runtime: Config) -> Config {
    Config {
        host: if let Some(h) = file.host {
            if h.is_empty() { defaults.host } else { Some(h) }
        } else {
            runtime.host.or(defaults.host)
        },
        port: file.port.or(runtime.port).or(defaults.port),
        debug: file.debug.or(runtime.debug).or(defaults.debug),
        workers: runtime.workers.or(file.workers).or(defaults.workers),
    }
}

fn main() {
    let defaults = Config {
        host: Some("localhost".to_string()),
        port: Some(3000),
        debug: Some(false),
        workers: Some(2),
    };

    let file = Config {
        host: Some(String::new()),
        port: Some(7000),
        debug: Some(false),
        workers: None,
    };

    let runtime = Config {
        host: None,
        port: Some(8080),
        debug: Some(true),
        workers: Some(4),
    };

    let merged = merge(defaults, file, runtime);
    println!("host={}", merged.host.unwrap_or_else(|| "<none>".to_string()));
    println!("port={}", merged.port.unwrap_or(0));
    println!("debug={}", merged.debug.unwrap_or(false));
    println!("workers={}", merged.workers.unwrap_or(0));
}
