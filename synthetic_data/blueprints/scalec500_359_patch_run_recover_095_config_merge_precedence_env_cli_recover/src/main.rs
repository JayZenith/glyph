struct Config {
    host: Option<&'static str>,
    port: Option<u16>,
    debug: Option<bool>,
}

fn merge(base: Config, file: Config, env: Config, cli: Config) -> Config {
    Config {
        host: cli.host.or(env.host).or(file.host).or(base.host),
        port: file.port.or(env.port).or(cli.port).or(base.port),
        debug: if cli.debug == Some(true) {
            Some(true)
        } else {
            env.debug.or(file.debug).or(base.debug)
        },
    }
}

fn main() {
    let defaults = Config {
        host: Some("localhost"),
        port: Some(8080),
        debug: Some(false),
    };
    let file = Config {
        host: Some("file.local"),
        port: Some(3000),
        debug: Some(false),
    };
    let env = Config {
        host: None,
        port: Some(9090),
        debug: Some(true),
    };
    let cli = Config {
        host: None,
        port: Some(7070),
        debug: Some(false),
    };

    let merged = merge(defaults, file, env, cli);
    println!(
        "{{\"host\":\"{}\",\"port\":{},\"debug\":{}}}",
        merged.host.unwrap(),
        merged.port.unwrap(),
        merged.debug.unwrap()
    );
}
