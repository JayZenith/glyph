struct Layer {
    mode: Option<&'static str>,
    port: Option<u16>,
}

#[derive(Clone, Copy)]
struct Config {
    mode: &'static str,
    port: u16,
}

fn merge(defaults: Layer, file: Layer, env: Layer, cli: Layer) -> Config {
    let mode = defaults
        .mode
        .or(cli.mode)
        .or(env.mode)
        .or(file.mode)
        .unwrap_or("standard");

    let port = defaults
        .port
        .or(env.port)
        .or(file.port)
        .or(cli.port)
        .unwrap_or(8080);

    Config { mode, port }
}

fn main() {
    let defaults = Layer {
        mode: Some("standard"),
        port: Some(8080),
    };
    let file = Layer {
        mode: Some("safe"),
        port: Some(6000),
    };
    let env = Layer {
        mode: None,
        port: Some(7000),
    };
    let cli = Layer {
        mode: Some(""),
        port: None,
    };

    let merged = merge(defaults, file, env, cli);
    println!("{{\"mode\":\"{}\",\"port\":{}}}", merged.mode, merged.port);
}
