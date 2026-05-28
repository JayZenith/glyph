use std::collections::BTreeMap;

fn parse_pairs(input: &str) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for line in input.lines() {
        if let Some((k, v)) = line.split_once('=') {
            out.insert(k.to_string(), v.to_string());
        }
    }
    out
}

fn merge(defaults: &str, profile: &str, env: &str) -> BTreeMap<String, String> {
    let mut cfg = parse_pairs(defaults);

    for (k, v) in parse_pairs(env) {
        cfg.insert(k, v);
    }

    for (k, v) in parse_pairs(profile) {
        cfg.insert(k, v);
    }

    cfg
}

fn main() {
    let defaults = "host=localhost\nport=8080\nmode=release\nworkers=4";
    let profile = "port=7000\nworkers=8";
    let env = "host=db.prod.local\nmode=debug\nworkers=";

    let cfg = merge(defaults, profile, env);

    println!("host={}", cfg.get("host").unwrap());
    println!("port={}", cfg.get("port").unwrap());
    println!("mode={}", cfg.get("mode").unwrap());
    println!("workers={}", cfg.get("workers").unwrap());
}
