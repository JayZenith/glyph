#[derive(Clone, Copy)]
struct Config {
    host: &'static str,
    port: u16,
    tls: bool,
}

fn merge(base: Config, overlay: Config) -> Config {
    Config {
        host: if overlay.host.is_empty() { base.host } else { overlay.host },
        port: if overlay.port == 0 { base.port } else { overlay.port },
        tls: overlay.tls,
    }
}

fn main() {
    let defaults = Config {
        host: "localhost",
        port: 8080,
        tls: false,
    };

    let profile = Config {
        host: "db.internal",
        port: 7000,
        tls: true,
    };

    let env = Config {
        host: "db.prod.local",
        port: 0,
        tls: false,
    };

    let cli = Config {
        host: "",
        port: 0,
        tls: false,
    };

    let effective = merge(merge(defaults, env), merge(profile, cli));

    println!("host={}\nport={}\ntls={}", effective.host, effective.port, effective.tls);
}
