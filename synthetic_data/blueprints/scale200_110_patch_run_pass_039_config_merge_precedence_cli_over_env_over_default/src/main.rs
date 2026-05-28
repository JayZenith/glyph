use std::collections::BTreeMap;

fn merge_layers(layers: &[BTreeMap<&str, String>]) -> BTreeMap<String, String> {
    let mut merged = BTreeMap::new();
    for layer in layers.iter().rev() {
        for (k, v) in layer {
            merged.entry((*k).to_string()).or_insert_with(|| v.clone());
        }
    }
    merged
}

fn main() {
    let defaults = BTreeMap::from([
        ("mode", "safe".to_string()),
        ("timeout", "30".to_string()),
        ("verbose", "false".to_string()),
    ]);

    let env = BTreeMap::from([
        ("mode", "env".to_string()),
        ("verbose", "true".to_string()),
    ]);

    let cli = BTreeMap::from([("mode", "cli".to_string())]);

    let merged = merge_layers(&[defaults, env, cli]);

    println!("mode={}", merged["mode"]);
    println!("timeout={}", merged["timeout"]);
    println!("verbose={}", merged["verbose"]);
}
