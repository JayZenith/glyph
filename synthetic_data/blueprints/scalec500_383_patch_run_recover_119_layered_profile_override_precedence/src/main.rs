use std::collections::BTreeMap;

#[derive(Clone)]
struct Config {
    values: BTreeMap<String, String>,
}

impl Config {
    fn from_pairs(pairs: &[(&str, &str)]) -> Self {
        let mut values = BTreeMap::new();
        for (k, v) in pairs {
            values.insert((*k).to_string(), (*v).to_string());
        }
        Self { values }
    }

    fn merge(base: &Config, overlay: &Config) -> Config {
        let mut values = overlay.values.clone();
        for (k, v) in &base.values {
            values.insert(k.clone(), v.clone());
        }
        Config { values }
    }
}

fn main() {
    let defaults = Config::from_pairs(&[
        ("timeout", "30"),
        ("retries", "2"),
        ("endpoint", "https://api.example.com"),
        ("enabled", "true"),
    ]);

    let profile = Config::from_pairs(&[
        ("timeout", "60"),
        ("endpoint", "https://stage.example.com"),
    ]);

    let env = Config::from_pairs(&[
        ("retries", "5"),
        ("enabled", "false"),
    ]);

    let merged = Config::merge(&env, &Config::merge(&profile, &defaults));

    for key in ["timeout", "retries", "endpoint", "enabled"] {
        println!("{}={}", key, merged.values.get(key).unwrap());
    }
}
