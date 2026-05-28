#[derive(Clone, Debug)]
struct Config {
    host: Option<String>,
    port: Option<u16>,
    debug: Option<bool>,
    tags: Vec<String>,
}

fn merge(base: Config, layer: Config) -> Config {
    Config {
        host: base.host.or(layer.host),
        port: base.port.or(layer.port),
        debug: base.debug.or(layer.debug),
        tags: {
            let mut tags = base.tags;
            tags.extend(layer.tags);
            tags
        },
    }
}

fn joined(tags: &[String]) -> String {
    tags.join(",")
}

fn main() {
    let defaults = Config {
        host: Some("localhost".to_string()),
        port: Some(8080),
        debug: Some(true),
        tags: vec!["base".to_string()],
    };

    let file_cfg = Config {
        host: Some("file.example.com".to_string()),
        port: None,
        debug: Some(false),
        tags: vec!["file".to_string(), "stable".to_string()],
    };

    let env_cfg = Config {
        host: Some("env.example.com".to_string()),
        port: Some(9000),
        debug: None,
        tags: vec!["ops".to_string(), "blue".to_string()],
    };

    let effective = merge(merge(defaults, file_cfg), env_cfg);

    println!("host={}", effective.host.unwrap_or_else(|| "<none>".to_string()));
    println!("port={}", effective.port.map(|p| p.to_string()).unwrap_or_else(|| "<none>".to_string()));
    println!("debug={}", effective.debug.map(|d| d.to_string()).unwrap_or_else(|| "<none>".to_string()));
    println!("tags={}", joined(&effective.tags));
}
