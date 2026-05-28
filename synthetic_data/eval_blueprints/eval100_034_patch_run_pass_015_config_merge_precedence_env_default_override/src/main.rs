use std::collections::BTreeMap;

#[derive(Clone, Debug)]
struct Config {
    host: String,
    port: u16,
    timeout: u32,
    debug: bool,
}

fn parse_pairs(input: &str) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for part in input.split(';').filter(|s| !s.is_empty()) {
        let (k, v) = part.split_once('=').unwrap();
        out.insert(k.to_string(), v.to_string());
    }
    out
}

fn merge_config(
    defaults: &BTreeMap<String, String>,
    file_cfg: &BTreeMap<String, String>,
    env_cfg: &BTreeMap<String, String>,
    cli_cfg: &BTreeMap<String, String>,
) -> Config {
    let host = defaults
        .get("host")
        .or(file_cfg.get("host"))
        .or(env_cfg.get("host"))
        .or(cli_cfg.get("host"))
        .unwrap()
        .clone();

    let port = defaults
        .get("port")
        .or(file_cfg.get("port"))
        .or(env_cfg.get("port"))
        .or(cli_cfg.get("port"))
        .unwrap()
        .parse()
        .unwrap();

    let timeout = defaults
        .get("timeout")
        .or(file_cfg.get("timeout"))
        .or(env_cfg.get("timeout"))
        .or(cli_cfg.get("timeout"))
        .unwrap()
        .parse()
        .unwrap();

    let debug = defaults
        .get("debug")
        .or(file_cfg.get("debug"))
        .or(env_cfg.get("debug"))
        .or(cli_cfg.get("debug"))
        .unwrap()
        == "true";

    Config {
        host,
        port,
        timeout,
        debug,
    }
}

fn main() {
    let defaults = parse_pairs("host=localhost;port=8080;timeout=30;debug=false");
    let file_cfg = parse_pairs("host=file.example;timeout=45");
    let env_cfg = parse_pairs("port=7000;debug=true");
    let cli_cfg = parse_pairs("port=9000");

    let cfg = merge_config(&defaults, &file_cfg, &env_cfg, &cli_cfg);

    println!("host={}", cfg.host);
    println!("port={}", cfg.port);
    println!("timeout={}", cfg.timeout);
    println!("debug={}", cfg.debug);
}
