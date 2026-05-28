use std::collections::BTreeMap;

fn parse_kv(input: &str) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            out.insert(k.trim().to_string(), v.trim().to_string());
        }
    }
    out
}

fn merge_layers(defaults: &str, env: &str, cli: &str) -> BTreeMap<String, String> {
    let mut merged = parse_kv(defaults);
    for (k, v) in parse_kv(cli) {
        merged.insert(k, v);
    }
    for (k, v) in parse_kv(env) {
        merged.insert(k, v);
    }
    merged
}

fn main() {
    let defaults = "mode=dev
host=default.local
port=8080
timeout=30
cache=true";
    let env = "mode=prod
timeout=45";
    let cli = "port=9090
cache=false";

    let merged = merge_layers(defaults, env, cli);
    let order = ["mode", "host", "port", "timeout", "cache"];

    for key in order {
        if let Some(value) = merged.get(key) {
            println!("{}={}", key, value);
        }
    }
}
