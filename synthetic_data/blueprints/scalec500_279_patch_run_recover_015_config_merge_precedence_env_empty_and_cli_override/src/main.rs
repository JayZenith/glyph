use std::collections::BTreeMap;

fn parse_pairs(s: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for part in s.split(';') {
        if part.is_empty() {
            continue;
        }
        if let Some((k, v)) = part.split_once('=') {
            map.insert(k.to_string(), v.to_string());
        }
    }
    map
}

fn merge_layers(layers: &[BTreeMap<String, String>]) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for layer in layers {
        for (k, v) in layer {
            out.insert(k.clone(), v.clone());
        }
    }
    out
}

fn main() {
    let defaults = parse_pairs("host=localhost;port=8080;mode=prod");
    let file_cfg = parse_pairs("host=db.internal;port=8000");
    let env_cfg = parse_pairs("host=;port=9000;mode=debug");
    let cli_cfg = parse_pairs("port=9090");

    let merged = merge_layers(&[defaults, cli_cfg, env_cfg, file_cfg]);

    println!("host={}", merged.get("host").unwrap());
    println!("port={}", merged.get("port").unwrap());
    println!("mode={}", merged.get("mode").unwrap());
}
