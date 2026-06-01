struct Config {
    mode: Option<&'static str>,
    retries: Option<u8>,
    color: Option<bool>,
    path: Option<&'static str>,
}

fn merged_value<T: Copy>(default: Option<T>, env: Option<T>, cli: Option<T>) -> Option<T> {
    cli.or(default).or(env)
}

fn main() {
    let defaults = Config {
        mode: Some("fast"),
        retries: Some(1),
        color: Some(false),
        path: Some("/default"),
    };

    let env = Config {
        mode: Some("slow"),
        retries: Some(3),
        color: Some(true),
        path: Some("/env"),
    };

    let cli = Config {
        mode: None,
        retries: None,
        color: None,
        path: Some("/cli"),
    };

    let mode = merged_value(defaults.mode, env.mode, cli.mode).unwrap();
    let retries = merged_value(defaults.retries, cli.retries, env.retries).unwrap();
    let color = cli.color.or(defaults.color).unwrap();
    let path = merged_value(defaults.path, env.path, cli.path).unwrap();

    println!("mode={}", mode);
    println!("retries={}", retries);
    println!("color={}", color);
    println!("path={}", path);
}
