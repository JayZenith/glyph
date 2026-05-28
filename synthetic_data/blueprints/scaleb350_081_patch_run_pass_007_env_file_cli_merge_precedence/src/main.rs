#[derive(Clone, Debug)]
struct Config {
    host: String,
    port: u16,
    mode: String,
    verbose: bool,
}

fn merge(defaults: Config, file: Option<Config>, cli: Option<Config>) -> Config {
    let mut cfg = defaults;

    if let Some(cli_cfg) = cli {
        cfg = cli_cfg;
    }

    if let Some(file_cfg) = file {
        cfg = file_cfg;
    }

    cfg
}

fn main() {
    let defaults = Config {
        host: "localhost".to_string(),
        port: 8080,
        mode: "prod".to_string(),
        verbose: false,
    };

    let file = Some(Config {
        host: "file.internal".to_string(),
        port: 9000,
        mode: "dev".to_string(),
        verbose: false,
    });

    let cli = Some(Config {
        host: "cli.example.com".to_string(),
        port: 0,
        mode: "".to_string(),
        verbose: true,
    });

    let cfg = merge(defaults, file, cli);
    println!(
        "{{\"host\":\"{}\",\"port\":{},\"mode\":\"{}\",\"verbose\":{}}}",
        cfg.host, cfg.port, cfg.mode, cfg.verbose
    );
}
