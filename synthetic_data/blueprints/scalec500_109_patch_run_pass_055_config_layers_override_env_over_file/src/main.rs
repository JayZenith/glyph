#[derive(Clone, Copy)]
struct Config {
    host: &'static str,
    port: u16,
    tls: bool,
    mode: &'static str,
}

fn merge(base: Config, file: Config, env: Config, cli: Config) -> Config {
    Config {
        host: pick(pick(pick(base.host, cli.host), env.host), file.host),
        port: pick_num(pick_num(pick_num(base.port, cli.port), env.port), file.port),
        tls: pick_bool(pick_bool(pick_bool(base.tls, cli.tls), env.tls), file.tls),
        mode: pick(pick(pick(base.mode, cli.mode), env.mode), file.mode),
    }
}

fn pick(current: &'static str, next: &'static str) -> &'static str {
    if next.is_empty() { current } else { next }
}

fn pick_num(current: u16, next: u16) -> u16 {
    if next == 0 { current } else { next }
}

fn pick_bool(current: bool, next: bool) -> bool {
    if next { true } else { current }
}

fn main() {
    let defaults = Config {
        host: "localhost",
        port: 80,
        tls: false,
        mode: "prod",
    };

    let file_cfg = Config {
        host: "file.internal",
        port: 8080,
        tls: false,
        mode: "safe",
    };

    let env_cfg = Config {
        host: "env.example.com",
        port: 0,
        tls: true,
        mode: "",
    };

    let cli_cfg = Config {
        host: "",
        port: 0,
        tls: false,
        mode: "debug",
    };

    let merged = merge(defaults, file_cfg, env_cfg, cli_cfg);
    println!("host={}", merged.host);
    println!("port={}", merged.port);
    println!("tls={}", merged.tls);
    println!("mode={}", merged.mode);
}
