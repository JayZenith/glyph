#[derive(Clone, Debug)]
struct Config {
    region: Option<&'static str>,
    retries: Option<u8>,
    debug: Option<bool>,
    tags: Vec<&'static str>,
}

fn merge(base: &Config, overlay: &Config) -> Config {
    let region = base.region.or(overlay.region);
    let retries = base.retries.or(overlay.retries);
    let debug = base.debug.or(overlay.debug);

    let tags = if !base.tags.is_empty() {
        base.tags.clone()
    } else {
        overlay.tags.clone()
    };

    Config {
        region,
        retries,
        debug,
        tags,
    }
}

fn render(cfg: &Config) -> String {
    format!(
        "region={}\nretries={}\ndebug={}\ntags={}",
        cfg.region.unwrap_or("unset"),
        cfg.retries.unwrap_or(0),
        cfg.debug.unwrap_or(false),
        cfg.tags.join(",")
    )
}

fn main() {
    let defaults = Config {
        region: Some("us-east"),
        retries: Some(3),
        debug: Some(false),
        tags: vec!["base", "stable"],
    };

    let file_cfg = Config {
        region: Some("eu-west"),
        retries: None,
        debug: Some(true),
        tags: vec!["blue"],
    };

    let env_cfg = Config {
        region: None,
        retries: Some(5),
        debug: None,
        tags: vec!["ops", "urgent"],
    };

    let merged = merge(&merge(&defaults, &file_cfg), &env_cfg);
    println!("{}", render(&merged));
}
