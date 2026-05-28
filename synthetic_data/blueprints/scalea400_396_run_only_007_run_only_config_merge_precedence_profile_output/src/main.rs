use std::collections::BTreeMap;

fn merge<'a>(layers: &[&'a BTreeMap<&'a str, &'a str>]) -> BTreeMap<&'a str, &'a str> {
    let mut out = BTreeMap::new();
    for layer in layers {
        for (k, v) in layer.iter() {
            out.insert(*k, *v);
        }
    }
    out
}

fn main() {
    let defaults = BTreeMap::from([
        ("host", "localhost"),
        ("port", "8080"),
        ("timeout", "30"),
        ("retries", "2"),
        ("profile", "dev"),
    ]);

    let env = BTreeMap::from([
        ("host", "staging.example.com"),
        ("timeout", "15"),
        ("profile", "staging"),
    ]);

    let cli = BTreeMap::from([
        ("port", "443"),
        ("retries", "5"),
    ]);

    let merged = merge(&[&defaults, &env, &cli]);
    let url = format!(
        "https://{}:{}",
        merged.get("host").unwrap(),
        merged.get("port").unwrap()
    );

    println!("final_url={}", url);
    println!("timeout={}", merged.get("timeout").unwrap());
    println!("retries={}", merged.get("retries").unwrap());
    println!("profile={}", merged.get("profile").unwrap());
}
