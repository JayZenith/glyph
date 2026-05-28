use std::collections::BTreeMap;

fn parse_pairs(input: &str) -> BTreeMap<String, String> {
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

fn merge_configs(defaults: &BTreeMap<String, String>, env: &BTreeMap<String, String>, cli: &BTreeMap<String, String>) -> BTreeMap<String, String> {
    let mut merged = cli.clone();
    for (k, v) in env {
        merged.entry(k.clone()).or_insert(v.clone());
    }
    for (k, v) in defaults {
        merged.entry(k.clone()).or_insert(v.clone());
    }
    merged
}

fn main() {
    let defaults = parse_pairs(
        "host=localhost
port=8080
timeout=30
verbose=false",
    );
    let env = parse_pairs(
        "host=env.internal
port=9000
verbose=true",
    );
    let cli = parse_pairs(
        "host=cli.example.com
timeout=15",
    );

    let merged = merge_configs(&defaults, &env, &cli);
    for key in ["host", "port", "timeout", "verbose"] {
        println!("{}={}", key, merged.get(key).unwrap());
    }
}
