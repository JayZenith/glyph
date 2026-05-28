#[derive(Clone, Debug)]
struct Config {
    host: String,
    port: u16,
    mode: String,
    retries: u8,
}

fn merge(default: &Config, file: &Config, env: &Config, cli: &Config) -> Config {
    let mut out = default.clone();

    if !file.host.is_empty() {
        out.host = file.host.clone();
    }
    if file.port != 0 {
        out.port = file.port;
    }
    if !file.mode.is_empty() {
        out.mode = file.mode.clone();
    }
    if file.retries != 0 {
        out.retries = file.retries;
    }

    if !cli.host.is_empty() {
        out.host = cli.host.clone();
    } else if !env.host.is_empty() {
        out.host = env.host.clone();
    }
    if cli.port != 0 {
        out.port = cli.port;
    } else if env.port != 0 {
        out.port = env.port;
    }
    if !cli.mode.is_empty() {
        out.mode = cli.mode.clone();
    } else if !env.mode.is_empty() {
        out.mode = env.mode.clone();
    }
    if cli.retries != 0 {
        out.retries = cli.retries;
    } else if env.retries != 0 {
        out.retries = env.retries;
    }

    out
}

fn main() {
    let defaults = Config {
        host: "localhost".into(),
        port: 8080,
        mode: "release".into(),
        retries: 3,
    };

    let file = Config {
        host: "file.local".into(),
        port: 7000,
        mode: "staging".into(),
        retries: 5,
    };

    let env = Config {
        host: "env.local".into(),
        port: 0,
        mode: "debug".into(),
        retries: 2,
    };

    let cli = Config {
        host: "".into(),
        port: 9000,
        mode: "".into(),
        retries: 0,
    };

    let merged = merge(&defaults, &file, &env, &cli);
    println!("host={}", merged.host);
    println!("port={}", merged.port);
    println!("mode={}", merged.mode);
    println!("retries={}", merged.retries);
}
