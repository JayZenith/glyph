use std::collections::BTreeMap;

fn parse_layer(s: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for part in s.split(';') {
        if part.is_empty() {
            continue;
        }
        let mut it = part.splitn(2, '=');
        let key = it.next().unwrap_or("").trim();
        let value = it.next().unwrap_or("").trim();
        if !key.is_empty() {
            map.insert(key.to_string(), value.to_string());
        }
    }
    map
}

fn merge_layers(layers: &[BTreeMap<String, String>]) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for layer in layers.iter().rev() {
        for (k, v) in layer {
            out.insert(k.clone(), v.clone());
        }
    }
    out
}

fn main() {
    let defaults = parse_layer("host=localhost;port=8080;timeout=30;debug=false");
    let file_cfg = parse_layer("host=file.internal;port=9000");
    let env_cfg = parse_layer("host=;port=7000;debug=true");
    let cli_cfg = parse_layer("timeout=");

    let merged = merge_layers(&[defaults, file_cfg, env_cfg, cli_cfg]);

    println!("host={}", merged.get("host").unwrap());
    println!("port={}", merged.get("port").unwrap());
    println!("timeout={}", merged.get("timeout").unwrap());
    println!("debug={}", merged.get("debug").unwrap());
}
