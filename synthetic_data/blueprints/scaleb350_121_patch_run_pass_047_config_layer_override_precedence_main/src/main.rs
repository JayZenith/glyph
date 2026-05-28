fn merge_configs(layers: &[&[(&str, &str)]]) -> Vec<(String, String)> {
    let mut merged: Vec<(String, String)> = Vec::new();

    for layer in layers {
        for (key, value) in *layer {
            if !merged.iter().any(|(k, _)| k == key) {
                merged.push((key.to_string(), value.to_string()));
            }
        }
    }

    merged
}

fn main() {
    let defaults = [("mode", "release"), ("timeout", "30"), ("retries", "1")];
    let env = [("timeout", "60"), ("retries", "3")];
    let cli = [("mode", "debug")];

    let merged = merge_configs(&[&defaults, &env, &cli]);

    for key in ["timeout", "mode", "retries"] {
        let value = merged
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
            .unwrap();
        println!("{}={}", key, value);
    }
}
