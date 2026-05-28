#[derive(Clone, Debug)]
struct Config {
    host: Option<&'static str>,
    port: Option<u16>,
    tls: Option<bool>,
    mode: Option<&'static str>,
}

fn merge(base: Config, env: Config, file: Config, cli: Config) -> Config {
    let mut out = base;

    if env.host.is_some() {
        out.host = env.host;
    }
    if env.port.is_some() {
        out.port = env.port;
    }
    if env.tls.is_some() {
        out.tls = env.tls;
    }
    if env.mode.is_some() {
        out.mode = env.mode;
    }

    if file.host.is_some() {
        out.host = file.host;
    }
    if file.port.is_some() {
        out.port = file.port;
    }
    if file.tls.is_some() {
        out.tls = file.tls;
    }
    if file.mode.is_some() {
        out.mode = file.mode;
    }

    if cli.host.is_some() {
        out.host = cli.host;
    }
    if cli.port.is_some() {
        out.port = cli.port;
    }
    if cli.tls.is_some() {
        out.tls = cli.tls;
    }
    if cli.mode.is_some() {
        out.mode = cli.mode;
    }

    out
}

fn main() {
    let defaults = Config {
        host: Some("localhost"),
        port: Some(80),
        tls: Some(false),
        mode: Some("prod"),
    };

    let env = Config {
        host: Some("env.example.com"),
        port: Some(3000),
        tls: Some(true),
        mode: None,
    };

    let file = Config {
        host: Some("file.example.com"),
        port: Some(8080),
        tls: None,
        mode: Some("debug"),
    };

    let cli = Config {
        host: Some("cli.example.com"),
        port: None,
        tls: Some(false),
        mode: Some("null"),
    };

    let merged = merge(defaults, env, file, cli);
    println!(
        "{{\"host\":\"{}\",\"port\":{},\"tls\":{},\"mode\":\"{}\"}}",
        merged.host.unwrap_or(""),
        merged.port.unwrap_or(0),
        merged.tls.unwrap_or(false),
        merged.mode.unwrap_or("")
    );
}
