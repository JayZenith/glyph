use std::collections::BTreeMap;

fn parse_pairs(input: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for part in input.split(';') {
        if part.is_empty() {
            continue;
        }
        let Some((k, v)) = part.split_once('=') else {
            continue;
        };
        map.insert(k.to_string(), v.to_string());
    }
    map
}

fn merge_config(defaults: &str, profile: &str, env: &str) -> BTreeMap<String, String> {
    let defaults = parse_pairs(defaults);
    let profile = parse_pairs(profile);
    let env = parse_pairs(env);
    let mut merged = BTreeMap::new();

    for (k, v) in defaults {
        merged.insert(k, v);
    }

    for (k, v) in env {
        if !v.is_empty() {
            merged.insert(k, v);
        }
    }

    for (k, v) in profile {
        merged.insert(k, v);
    }

    merged
}

fn main() {
    let defaults = "host=localhost;port=8080;mode=prod;log=info";
    let profile = "host=profile.local;mode=dev";
    let env = "port=9000;mode=;log=debug";

    let merged = merge_config(defaults, profile, env);
    println!("host={}", merged.get("host").unwrap());
    println!("port={}", merged.get("port").unwrap());
    println!("mode={}", merged.get("mode").unwrap());
    println!("log={}", merged.get("log").unwrap());
}
