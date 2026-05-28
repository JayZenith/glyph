#[derive(Clone, Copy)]
struct Config {
    host: &'static str,
    port: u16,
    tls: bool,
    mode: &'static str,
}

fn merge(base: Config, layer: Config) -> Config {
    Config {
        host: if layer.host.is_empty() { base.host } else { layer.host },
        port: if layer.port == 0 { base.port } else { layer.port },
        tls: base.tls,
        mode: if layer.mode.is_empty() { base.mode } else { layer.mode },
    }
}

fn main() {
    let defaults = Config {
        host: "localhost",
        port: 8080,
        tls: false,
        mode: "prod",
    };
    let env = Config {
        host: "env.internal",
        port: 9000,
        tls: true,
        mode: "",
    };
    let file = Config {
        host: "file.service",
        port: 7000,
        tls: false,
        mode: "dev",
    };
    let cli = Config {
        host: "cli.example.com",
        port: 0,
        tls: false,
        mode: "",
    };

    let cfg = merge(merge(merge(defaults, cli), file), env);

    println!("host={}", cfg.host);
    println!("port={}", cfg.port);
    println!("tls={}", cfg.tls);
    print!("mode={}", cfg.mode);
}
