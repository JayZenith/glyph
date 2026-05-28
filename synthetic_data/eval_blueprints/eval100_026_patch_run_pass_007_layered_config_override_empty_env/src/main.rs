use std::collections::BTreeMap;

fn merge_configs(
    defaults: &BTreeMap<&str, &str>,
    file_cfg: &BTreeMap<&str, &str>,
    env_cfg: &BTreeMap<&str, &str>,
    cli_cfg: &BTreeMap<&str, &str>,
) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();

    for (k, v) in defaults {
        out.insert((*k).to_string(), (*v).to_string());
    }
    for (k, v) in file_cfg {
        out.insert((*k).to_string(), (*v).to_string());
    }
    for (k, v) in env_cfg {
        out.insert((*k).to_string(), (*v).to_string());
    }
    for (k, v) in cli_cfg {
        if !v.is_empty() {
            out.insert((*k).to_string(), (*v).to_string());
        }
    }

    out
}

fn main() {
    let defaults = BTreeMap::from([
        ("host", "localhost"),
        ("port", "5432"),
        ("tls", "false"),
    ]);
    let file_cfg = BTreeMap::from([
        ("host", "db.internal"),
        ("port", "6000"),
    ]);
    let env_cfg = BTreeMap::from([
        ("host", ""),
        ("port", "7000"),
        ("tls", "true"),
    ]);
    let cli_cfg = BTreeMap::from([
        ("host", ""),
    ]);

    let merged = merge_configs(&defaults, &file_cfg, &env_cfg, &cli_cfg);
    println!("host={}", merged.get("host").unwrap());
    println!("port={}", merged.get("port").unwrap());
    println!("tls={}", merged.get("tls").unwrap());
}
