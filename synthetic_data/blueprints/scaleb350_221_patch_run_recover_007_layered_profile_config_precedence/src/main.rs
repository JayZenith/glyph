#[derive(Clone, Copy)]
struct Config {
    endpoint: Option<&'static str>,
    retries: Option<u8>,
    timeout: Option<u16>,
    verbose: Option<bool>,
}

fn merge(base: Config, overlay: Config) -> Config {
    Config {
        endpoint: base.endpoint.or(overlay.endpoint),
        retries: base.retries.or(overlay.retries),
        timeout: base.timeout.or(overlay.timeout),
        verbose: base.verbose.or(overlay.verbose),
    }
}

fn main() {
    let defaults = Config {
        endpoint: Some("https://api.dev.example"),
        retries: Some(2),
        timeout: Some(30),
        verbose: Some(false),
    };

    let profile = Config {
        endpoint: Some("https://api.prod.example"),
        retries: None,
        timeout: Some(45),
        verbose: None,
    };

    let cli = Config {
        endpoint: None,
        retries: Some(5),
        timeout: Some(60),
        verbose: Some(true),
    };

    let merged = merge(cli, merge(profile, defaults));

    println!(
        "{{\"endpoint\":\"{}\",\"retries\":{},\"timeout\":{},\"verbose\":{}}}",
        merged.endpoint.unwrap_or(""),
        merged.retries.unwrap_or(0),
        merged.timeout.unwrap_or(0),
        merged.verbose.unwrap_or(false)
    );
}
