#[derive(Clone, Copy)]
struct Config {
    region: &'static str,
    retries: u8,
    verbose: bool,
    cache: bool,
}

fn merge(base: Config, overlay: Config) -> Config {
    Config {
        region: if overlay.region.is_empty() { base.region } else { overlay.region },
        retries: if overlay.retries == 0 { base.retries } else { overlay.retries },
        verbose: base.verbose || overlay.verbose,
        cache: if overlay.cache { true } else { base.cache },
    }
}

fn main() {
    let defaults = Config {
        region: "us-east",
        retries: 3,
        verbose: false,
        cache: true,
    };

    let profile = Config {
        region: "eu-west",
        retries: 0,
        verbose: true,
        cache: false,
    };

    let cli = Config {
        region: "",
        retries: 5,
        verbose: false,
        cache: false,
    };

    let merged = merge(cli, merge(profile, defaults));

    println!(
        "region={} retries={} verbose={} cache={}",
        merged.region, merged.retries, merged.verbose, merged.cache
    );
}
