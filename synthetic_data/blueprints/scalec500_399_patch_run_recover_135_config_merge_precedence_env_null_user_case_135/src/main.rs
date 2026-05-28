#[derive(Clone, Copy)]
struct Config {
    host: Option<&'static str>,
    port: Option<u16>,
    tls: Option<bool>,
    mode: Option<&'static str>,
}

fn choose<T: Copy>(default: Option<T>, env: Option<T>, user: Option<T>) -> Option<T> {
    if let Some(v) = user {
        Some(v)
    } else if let Some(v) = default {
        Some(v)
    } else {
        env
    }
}

fn merge(default: Config, env: Config, user: Config) -> Config {
    Config {
        host: choose(default.host, env.host, user.host),
        port: choose(default.port, env.port, user.port),
        tls: choose(default.tls, env.tls, user.tls),
        mode: choose(default.mode, env.mode, user.mode),
    }
}

fn main() {
    let default = Config {
        host: Some("localhost"),
        port: Some(5432),
        tls: Some(false),
        mode: Some("readwrite"),
    };

    let env = Config {
        host: Some("db.example"),
        port: Some(6000),
        tls: Some(true),
        mode: None,
    };

    let user = Config {
        host: None,
        port: Some(7000),
        tls: None,
        mode: Some("readonly"),
    };

    let merged = merge(default, env, user);
    println!("host={}", merged.host.unwrap_or("-"));
    println!("port={}", merged.port.unwrap_or(0));
    println!("tls={}", merged.tls.unwrap_or(false));
    println!("mode={}", merged.mode.unwrap_or("-"));
}
