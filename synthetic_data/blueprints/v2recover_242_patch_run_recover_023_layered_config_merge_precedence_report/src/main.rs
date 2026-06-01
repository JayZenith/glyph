#[derive(Clone, Copy)]
struct Config<'a> {
    mode: Option<&'a str>,
    retries: Option<u8>,
    endpoint: Option<&'a str>,
}

fn merge<'a>(defaults: Config<'a>, file: Config<'a>, env: Config<'a>) -> Config<'a> {
    Config {
        mode: file.mode.or(defaults.mode),
        retries: env.retries.or(defaults.retries),
        endpoint: defaults.endpoint.or(file.endpoint).or(env.endpoint),
    }
}

fn render(cfg: Config<'_>) -> String {
    format!(
        "mode={}\nretries={}\nendpoint={}",
        cfg.mode.unwrap_or("unknown"),
        cfg.retries.unwrap_or(0),
        cfg.endpoint.unwrap_or("none")
    )
}

fn main() {
    let defaults = Config {
        mode: Some("safe"),
        retries: Some(3),
        endpoint: Some("https://default.example/api"),
    };

    let file = Config {
        mode: Some("fast"),
        retries: Some(5),
        endpoint: None,
    };

    let env = Config {
        mode: None,
        retries: Some(0),
        endpoint: Some("https://override.example/api"),
    };

    let effective = merge(defaults, file, env);
    println!("{}", render(effective));
}
