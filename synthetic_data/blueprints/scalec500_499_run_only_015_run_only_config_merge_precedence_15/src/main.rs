use std::collections::BTreeMap;

fn parse(input: &str) -> BTreeMap<String, String> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            line.split_once('=').map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        })
        .collect()
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
    let defaults = parse(
        "db=localhost:5432
cache=false
workers=4
mode=auto",
    );
    let env = parse(
        "db=db.internal:5432
workers=8",
    );
    let cli = parse(
        "db=replica.local:6000
cache=true
mode=manual",
    );

    let merged = merge_layers(&[defaults, env, cli]);
    for key in ["db", "cache", "workers", "mode"] {
        println!("{}={}", key, merged.get(key).unwrap());
    }
}
