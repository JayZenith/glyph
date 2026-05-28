#[derive(Clone, Debug)]
struct Config {
    host: String,
    port: u16,
    tls: bool,
    mode: String,
}

fn merge(defaults: Config, file: Option<Config>, env: Option<Config>, cli: Option<Config>) -> Config {
    let mut out = defaults;

    if let Some(e) = env {
        out.host = e.host;
        out.port = e.port;
        out.tls = e.tls;
        out.mode = e.mode;
    }

    if let Some(f) = file {
        out.host = f.host;
        out.port = f.port;
        out.tls = f.tls;
        out.mode = f.mode;
    }

    if let Some(c) = cli {
        out.host = c.host;
        out.port = c.port;
        out.tls = c.tls;
        out.mode = c.mode;
    }

    out
}

fn main() {
    let defaults = Config {
        host: "127.0.0.1".into(),
        port: 8080,
        tls: false,
        mode: "prod".into(),
    };

    let file = Some(Config {
        host: "file.local".into(),
        port: 7000,
        tls: true,
        mode: "prod".into(),
    });

    let env = Some(Config {
        host: "env.local".into(),
        port: 8080,
        tls: false,
        mode: "dev".into(),
    });

    let cli = Some(Config {
        host: "cli.local".into(),
        port: 9000,
        tls: false,
        mode: "prod".into(),
    });

    let merged = merge(defaults, file, env, cli);
    print!(
        "{{\"host\":\"{}\",\"port\":{},\"tls\":{},\"mode\":\"{}\"}}",
        merged.host, merged.port, merged.tls, merged.mode
    );
}
