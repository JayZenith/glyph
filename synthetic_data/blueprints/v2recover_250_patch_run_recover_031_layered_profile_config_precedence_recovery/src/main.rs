#[derive(Clone, Copy)]
struct Features {
    cache: bool,
    beta: bool,
}

#[derive(Clone, Copy)]
struct Config {
    profile: &'static str,
    region: &'static str,
    retries: u8,
    features: Features,
}

fn merge(base: Config, overlay: Config) -> Config {
    Config {
        profile: if overlay.profile.is_empty() { base.profile } else { overlay.profile },
        region: if overlay.region.is_empty() { base.region } else { overlay.region },
        retries: if overlay.retries == 0 { base.retries } else { overlay.retries },
        features: overlay.features,
    }
}

fn main() {
    let defaults = Config {
        profile: "default",
        region: "us-east",
        retries: 2,
        features: Features { cache: false, beta: false },
    };

    let dev = Config {
        profile: "dev",
        region: "local",
        retries: 1,
        features: Features { cache: true, beta: false },
    };

    let prod = Config {
        profile: "prod",
        region: "",
        retries: 4,
        features: Features { cache: true, beta: false },
    };

    let env = Config {
        profile: "",
        region: "ap-south",
        retries: 0,
        features: Features { cache: false, beta: true },
    };

    let cli = Config {
        profile: "prod",
        region: "eu-west",
        retries: 5,
        features: Features { cache: false, beta: false },
    };

    let selected = if env.profile == "prod" { prod } else { dev };
    let effective = merge(merge(merge(defaults, env), selected), cli);

    println!("profile={}", effective.profile);
    println!("region={}", effective.region);
    println!("retries={}", effective.retries);
    println!("cache={}", effective.features.cache);
    println!("beta={}", effective.features.beta);
}
