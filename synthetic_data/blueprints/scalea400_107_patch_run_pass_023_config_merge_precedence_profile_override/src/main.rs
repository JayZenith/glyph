#[derive(Clone, Debug)]
struct Config {
    host: String,
    port: u16,
    tls: bool,
    mode: String,
}

impl Config {
    fn defaults() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
            tls: false,
            mode: "prod".to_string(),
        }
    }

    fn merge(&self, other: &PartialConfig) -> Self {
        Self {
            host: other.host.clone().unwrap_or_else(|| self.host.clone()),
            port: other.port.unwrap_or(self.port),
            tls: other.tls.unwrap_or(self.tls),
            mode: other.mode.clone().unwrap_or_else(|| self.mode.clone()),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct PartialConfig {
    host: Option<String>,
    port: Option<u16>,
    tls: Option<bool>,
    mode: Option<String>,
}

fn main() {
    let defaults = Config::defaults();

    let env = PartialConfig {
        host: Some("env.local".to_string()),
        port: Some(9000),
        tls: Some(true),
        mode: None,
    };

    let profile = PartialConfig {
        host: Some("profile.local".to_string()),
        port: None,
        tls: None,
        mode: Some("debug".to_string()),
    };

    let cli = PartialConfig {
        host: None,
        port: Some(7000),
        tls: None,
        mode: None,
    };

    let effective = defaults
        .merge(&profile)
        .merge(&env)
        .merge(&cli);

    println!("host={}", effective.host);
    println!("port={}", effective.port);
    println!("tls={}", effective.tls);
    println!("mode={}", effective.mode);
}
