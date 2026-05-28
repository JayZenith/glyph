#[derive(Clone, Debug)]
struct Config {
    host: String,
    port: u16,
    mode: String,
    features: Vec<String>,
}

fn merge(base: Config, file: Config, env: Config, cli: Config) -> Config {
    let mut out = base.clone();

    out.host = cli.host;
    out.port = cli.port;
    out.mode = cli.mode;
    out.features = cli.features;

    if out.host.is_empty() {
        out.host = env.host;
    }
    if out.port == 0 {
        out.port = env.port;
    }
    if out.mode.is_empty() {
        out.mode = env.mode;
    }
    if out.features.is_empty() {
        out.features = env.features;
    }

    if out.host.is_empty() {
        out.host = file.host;
    }
    if out.port == 0 {
        out.port = file.port;
    }
    if out.mode.is_empty() {
        out.mode = file.mode;
    }
    if out.features.is_empty() {
        out.features = file.features;
    }

    out
}

fn main() {
    let defaults = Config {
        host: "localhost".to_string(),
        port: 8080,
        mode: "release".to_string(),
        features: vec!["base".to_string()],
    };

    let file = Config {
        host: "file.local".to_string(),
        port: 9000,
        mode: "staging".to_string(),
        features: vec!["base".to_string(), "cache".to_string()],
    };

    let env = Config {
        host: "".to_string(),
        port: 7000,
        mode: "debug".to_string(),
        features: vec![],
    };

    let cli = Config {
        host: "".to_string(),
        port: 0,
        mode: "".to_string(),
        features: vec!["trace".to_string()],
    };

    let effective = merge(defaults, file, env, cli);

    println!("host={}", effective.host);
    println!("port={}", effective.port);
    println!("mode={}", effective.mode);
    println!("features={}", effective.features.join(","));
}
