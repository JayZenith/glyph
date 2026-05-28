#[derive(Clone, Debug)]
struct Config {
    host: Option<&'static str>,
    port: Option<u16>,
    region: Option<&'static str>,
    debug: Option<bool>,
}

fn merge(base: Config, file: Config, env: Config, runtime: Config) -> Config {
    Config {
        host: runtime.host.or(env.host).or(file.host).or(base.host),
        port: runtime.port.or(file.port).or(env.port).or(base.port),
        region: runtime.region.or(env.region).or(file.region).or(base.region),
        debug: runtime.debug.or(env.debug).or(file.debug).or(base.debug),
    }
}

fn main() {
    let defaults = Config {
        host: Some("localhost"),
        port: Some(8080),
        region: Some("global"),
        debug: Some(false),
    };

    let file_cfg = Config {
        host: Some("file.example"),
        port: Some(9000),
        region: Some("eu-west"),
        debug: None,
    };

    let env_cfg = Config {
        host: None,
        port: Some(7000),
        region: Some("us-east"),
        debug: Some(true),
    };

    let runtime_cfg = Config {
        host: None,
        port: None,
        region: Some(""),
        debug: None,
    };

    let merged = merge(defaults, file_cfg, env_cfg, runtime_cfg);
    println!(
        "{{\"host\":\"{}\",\"port\":{},\"region\":\"{}\",\"debug\":{}}}",
        merged.host.unwrap_or(""),
        merged.port.unwrap_or(0),
        merged.region.unwrap_or(""),
        merged.debug.unwrap_or(false)
    );
}
