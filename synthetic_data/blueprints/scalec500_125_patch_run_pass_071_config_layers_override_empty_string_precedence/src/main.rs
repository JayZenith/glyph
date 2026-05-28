#[derive(Clone, Debug)]
struct Config {
    service: Option<String>,
    region: Option<String>,
    timeout: Option<u32>,
    debug: Option<bool>,
}

impl Config {
    fn empty() -> Self {
        Self {
            service: None,
            region: None,
            timeout: None,
            debug: None,
        }
    }
}

fn merge(defaults: &Config, file: &Config, env: &Config, cli: &Config) -> Config {
    Config {
        service: defaults
            .service
            .clone()
            .or(file.service.clone())
            .or(env.service.clone())
            .or(cli.service.clone()),
        region: defaults
            .region
            .clone()
            .or(file.region.clone())
            .or(env.region.clone())
            .or(cli.region.clone()),
        timeout: defaults.timeout.or(file.timeout).or(env.timeout).or(cli.timeout),
        debug: defaults.debug.or(file.debug).or(env.debug).or(cli.debug),
    }
}

fn main() {
    let defaults = Config {
        service: Some("api".to_string()),
        region: Some("us-east".to_string()),
        timeout: Some(30),
        debug: Some(false),
    };

    let file = Config {
        service: Some("".to_string()),
        region: Some("eu-west".to_string()),
        timeout: Some(45),
        debug: Some(true),
    };

    let env = Config {
        service: Some("".to_string()),
        region: None,
        timeout: None,
        debug: None,
    };

    let cli = Config {
        service: None,
        region: Some("".to_string()),
        timeout: None,
        debug: Some(false),
    };

    let merged = merge(&defaults, &file, &env, &cli);

    println!("service={}", merged.service.unwrap_or_else(|| "<none>".to_string()));
    println!("region={}", merged.region.unwrap_or_else(|| "<none>".to_string()));
    println!("timeout={}", merged.timeout.unwrap_or(0));
    println!("debug={}", merged.debug.unwrap_or(false));
}
