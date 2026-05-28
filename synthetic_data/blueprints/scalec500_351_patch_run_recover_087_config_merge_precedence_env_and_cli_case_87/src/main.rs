use std::collections::BTreeMap;

fn parse_pairs(input: &str) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for part in input.split(';') {
        if let Some((k, v)) = part.split_once('=') {
            out.insert(k.trim().to_string(), v.trim().to_string());
        }
    }
    out
}

fn merge(base: &BTreeMap<String, String>, env: &BTreeMap<String, String>, cli: &BTreeMap<String, String>) -> BTreeMap<String, String> {
    let mut out = cli.clone();
    for (k, v) in env {
        out.entry(k.clone()).or_insert_with(|| v.clone());
    }
    for (k, v) in base {
        out.entry(k.clone()).or_insert_with(|| v.clone());
    }
    out
}

fn main() {
    let defaults = parse_pairs("host=localhost;port=8080;debug=false;retries=2");
    let env_cfg = parse_pairs("host=env.example.com;debug=true");
    let cli_cfg = parse_pairs("port=9090;retries=5");

    let merged = merge(&defaults, &env_cfg, &cli_cfg);

    println!("host={}", merged.get("host").unwrap());
    println!("port={}", merged.get("port").unwrap());
    println!("debug={}", merged.get("debug").unwrap());
    println!("retries={}", merged.get("retries").unwrap());
}
