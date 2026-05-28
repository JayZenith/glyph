#[derive(Clone, Copy)]
struct Config {
    mode: &'static str,
    retries: u8,
    endpoint: &'static str,
}

fn merge(base: Config, profile: Config, env: Config, cli: Config) -> Config {
    let mut out = base;

    if profile.mode != "" {
        out.mode = profile.mode;
    }
    if env.mode != "" {
        out.mode = env.mode;
    }
    if cli.mode != "" {
        out.mode = cli.mode;
    }

    if profile.retries != 0 {
        out.retries = profile.retries;
    }
    if cli.retries != 0 {
        out.retries = cli.retries;
    }
    if env.retries != 0 {
        out.retries = env.retries;
    }

    if cli.endpoint != "" {
        out.endpoint = cli.endpoint;
    }
    if profile.endpoint != "" {
        out.endpoint = profile.endpoint;
    }
    if env.endpoint != "" {
        out.endpoint = env.endpoint;
    }

    out
}

fn main() {
    let base = Config {
        mode: "safe",
        retries: 1,
        endpoint: "https://default/api",
    };
    let profile = Config {
        mode: "fast",
        retries: 2,
        endpoint: "https://profile/api",
    };
    let env = Config {
        mode: "",
        retries: 5,
        endpoint: "https://env/api",
    };
    let cli = Config {
        mode: "",
        retries: 0,
        endpoint: "https://cli/api",
    };

    let merged = merge(base, profile, env, cli);
    println!(
        "mode={} retries={} endpoint={}",
        merged.mode, merged.retries, merged.endpoint
    );
}
