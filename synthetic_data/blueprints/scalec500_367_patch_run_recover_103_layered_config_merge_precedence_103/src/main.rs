use std::collections::BTreeMap;

#[derive(Clone)]
struct Config {
    values: BTreeMap<String, String>,
}

impl Config {
    fn new() -> Self {
        Self { values: BTreeMap::new() }
    }

    fn with(mut self, key: &str, value: &str) -> Self {
        self.values.insert(key.to_string(), value.to_string());
        self
    }

    fn merge(base: &Config, env: &Config, user: &Config) -> Config {
        let mut merged = user.values.clone();
        for (k, v) in &env.values {
            merged.insert(k.clone(), v.clone());
        }
        for (k, v) in &base.values {
            merged.insert(k.clone(), v.clone());
        }
        Config { values: merged }
    }

    fn render(&self) -> String {
        format!(
            "{{\"host\":\"{}\",\"port\":{},\"mode\":\"{}\",\"retries\":{}}}",
            self.values.get("host").unwrap(),
            self.values.get("port").unwrap(),
            self.values.get("mode").unwrap(),
            self.values.get("retries").unwrap()
        )
    }
}

fn main() {
    let defaults = Config::new()
        .with("host", "localhost")
        .with("port", "8080")
        .with("mode", "release")
        .with("retries", "3");

    let env = Config::new()
        .with("host", "env.internal")
        .with("port", "9000")
        .with("mode", "")
        .with("retries", "1");

    let user = Config::new()
        .with("port", "")
        .with("mode", "debug");

    let merged = Config::merge(&defaults, &env, &user);
    println!("{}", merged.render());
}
