use std::fmt;

#[derive(Clone, Debug, Default)]
struct Config {
    port: Option<u16>,
    mode: Option<String>,
    debug: Option<bool>,
    features: Option<Vec<String>>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let features = self.features.clone().unwrap_or_default().join(",");
        write!(
            f,
            "port={}\nmode={}\ndebug={}\nfeatures={}",
            self.port.unwrap_or(0),
            self.mode.clone().unwrap_or_default(),
            self.debug.unwrap_or(false),
            features
        )
    }
}

fn merge(base: Config, env: Config, cli: Config) -> Config {
    Config {
        port: base.port.or(env.port).or(cli.port),
        mode: base.mode.or(env.mode).or(cli.mode),
        debug: base.debug.or(env.debug).or(cli.debug),
        features: cli.features.or(env.features).or(base.features),
    }
}

fn main() {
    let defaults = Config {
        port: Some(3000),
        mode: Some("dev".to_string()),
        debug: Some(false),
        features: Some(vec!["base".to_string(), "cache".to_string()]),
    };

    let env = Config {
        port: Some(8080),
        mode: Some("staging".to_string()),
        debug: Some(true),
        features: None,
    };

    let cli = Config {
        port: None,
        mode: None,
        debug: Some(false),
        features: Some(vec![
            "base".to_string(),
            "cache".to_string(),
            "metrics".to_string(),
        ]),
    };

    let merged = merge(defaults, env, cli);
    println!("{}", merged);
}
