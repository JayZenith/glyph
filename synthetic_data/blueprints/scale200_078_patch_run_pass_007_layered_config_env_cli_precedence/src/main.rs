#[derive(Clone, Debug)]
struct Config {
    host: String,
    port: u16,
    mode: String,
    retries: u8,
}

fn merge(
    defaults: Config,
    file_cfg: Option<Config>,
    env_cfg: Option<Config>,
    cli_cfg: Option<Config>,
) -> Config {
    let mut cfg = defaults;

    // BUG: applies sources in the wrong order, letting lower-priority layers overwrite higher-priority ones.
    for layer in [cli_cfg, env_cfg, file_cfg] {
        if let Some(next) = layer {
            cfg = next;
        }
    }

    cfg
}

fn main() {
    let defaults = Config {
        host: "localhost".to_string(),
        port: 3000,
        mode: "prod".to_string(),
        retries: 2,
    };

    let file_cfg = Some(Config {
        host: "file.example.com".to_string(),
        port: 7000,
        mode: "prod".to_string(),
        retries: 3,
    });

    let env_cfg = Some(Config {
        host: "env.example.com".to_string(),
        port: 7000,
        mode: "debug".to_string(),
        retries: 3,
    });

    let cli_cfg = Some(Config {
        host: "env.example.com".to_string(),
        port: 8080,
        mode: "debug".to_string(),
        retries: 5,
    });

    let cfg = merge(defaults, file_cfg, env_cfg, cli_cfg);
    println!(
        "{{\"host\":\"{}\",\"port\":{},\"mode\":\"{}\",\"retries\":{}}}",
        cfg.host, cfg.port, cfg.mode, cfg.retries
    );
}
