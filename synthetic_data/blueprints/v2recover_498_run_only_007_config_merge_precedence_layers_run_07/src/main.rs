use std::collections::BTreeMap;

fn parse_pairs(s: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for part in s.split(';').filter(|p| !p.is_empty()) {
        let (k, v) = part.split_once('=').unwrap();
        map.insert(k.to_string(), v.to_string());
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
    let defaults = parse_pairs("host=localhost;port=80;mode=prod;timeout=30");
    let file_cfg = parse_pairs("host=file.example;port=8080;timeout=45");
    let cli_cfg = parse_pairs("mode=cli");

    let merged = merge_layers(&[defaults, file_cfg, cli_cfg]);
    for key in ["host", "port", "mode", "timeout"] {
        println!("{}={}", key, merged.get(key).unwrap());
    }
}
