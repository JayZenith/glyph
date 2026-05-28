#[derive(Clone, Copy)]
struct Config {
    host: &'static str,
    port: u16,
    tls: bool,
    timeout: u32,
}

fn merge(base: Config, profile: Option<Config>, env: Option<Config>, cli: Option<Config>) -> Config {
    let mut out = base;

    if let Some(c) = profile {
        out = c;
    }
    if let Some(c) = cli {
        out = c;
    }
    if let Some(c) = env {
        out = c;
    }

    out
}

fn main() {
    let defaults = Config {
        host: "default.local",
        port: 8080,
        tls: false,
        timeout: 30,
    };

    let profile = Some(Config {
        host: "profile.local",
        port: 9000,
        tls: true,
        timeout: 25,
    });

    let env = Some(Config {
        host: "default.local",
        port: 7000,
        tls: false,
        timeout: 45,
    });

    let cli = Some(Config {
        host: "default.local",
        port: 8080,
        tls: true,
        timeout: 30,
    });

    let effective = merge(defaults, profile, env, cli);

    print!(
        "{{\"host\":\"{}\",\"port\":{},\"tls\":{},\"timeout\":{}}}",
        effective.host, effective.port, effective.tls, effective.timeout
    );
}
