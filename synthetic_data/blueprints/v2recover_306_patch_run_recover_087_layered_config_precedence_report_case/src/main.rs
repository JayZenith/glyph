#[derive(Clone, Debug)]
struct Config {
    host: Option<String>,
    port: Option<u16>,
    mode: Option<String>,
    features: Vec<String>,
}

fn merge(base: &Config, overlay: &Config) -> Config {
    Config {
        host: base.host.clone().or_else(|| overlay.host.clone()),
        port: base.port.or(overlay.port),
        mode: base.mode.clone().or_else(|| overlay.mode.clone()),
        features: {
            let mut out = overlay.features.clone();
            out.extend(base.features.iter().cloned());
            out
        },
    }
}

fn main() {
    let defaults = Config {
        host: Some("localhost".to_string()),
        port: Some(8080),
        mode: Some("release".to_string()),
        features: vec!["metrics".to_string(), "trace".to_string()],
    };

    let env = Config {
        host: Some("env.internal".to_string()),
        port: Some(9090),
        mode: Some("debug".to_string()),
        features: vec!["trace".to_string(), "cache".to_string()],
    };

    let cli = Config {
        host: Some("".to_string()),
        port: None,
        mode: None,
        features: vec!["metrics".to_string()],
    };

    let merged = merge(&cli, &merge(&env, &defaults));

    println!("host={}", merged.host.unwrap_or_else(|| "-".to_string()));
    println!("port={}", merged.port.unwrap_or(0));
    println!("mode={}", merged.mode.unwrap_or_else(|| "-".to_string()));
    println!("features={}", merged.features.join(","));
}
