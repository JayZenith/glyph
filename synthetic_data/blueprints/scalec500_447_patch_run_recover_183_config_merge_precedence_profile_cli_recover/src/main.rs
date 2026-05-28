#[derive(Clone, Copy)]
struct Cfg<'a> {
    host: &'a str,
    port: u16,
    timeout: u16,
    mode: &'a str,
}

fn merge<'a>(base: Cfg<'a>, overlay: Cfg<'a>) -> Cfg<'a> {
    Cfg {
        host: if base.host.is_empty() { overlay.host } else { base.host },
        port: if overlay.port != 0 { overlay.port } else { base.port },
        timeout: if overlay.timeout != 0 { overlay.timeout } else { base.timeout },
        mode: if base.mode.is_empty() { overlay.mode } else { base.mode },
    }
}

fn main() {
    let defaults = Cfg {
        host: "localhost",
        port: 8080,
        timeout: 30,
        mode: "dev",
    };

    let profile = Cfg {
        host: "profile.internal",
        port: 0,
        timeout: 45,
        mode: "prod",
    };

    let env = Cfg {
        host: "env.example.com",
        port: 9090,
        timeout: 0,
        mode: "",
    };

    let cli = Cfg {
        host: "cli.example.com",
        port: 0,
        timeout: 0,
        mode: "",
    };

    let combined = merge(cli, merge(env, merge(profile, defaults)));

    println!("host={}", combined.host);
    println!("port={}", combined.port);
    println!("timeout={}", combined.timeout);
    println!("mode={}", combined.mode);
}
