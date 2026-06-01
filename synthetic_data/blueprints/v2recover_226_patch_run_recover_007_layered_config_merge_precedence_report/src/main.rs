#[derive(Clone, Debug)]
struct Config {
    mode: Option<&'static str>,
    threads: Option<u16>,
    path: Option<&'static str>,
    verbose: Option<bool>,
}

fn merge(base: Config, overlay: Config) -> Config {
    Config {
        mode: base.mode.or(overlay.mode),
        threads: base.threads.or(overlay.threads),
        path: base.path.or(overlay.path),
        verbose: base.verbose.or(overlay.verbose),
    }
}

fn main() {
    let defaults = Config {
        mode: Some("release"),
        threads: Some(4),
        path: Some("/opt/service"),
        verbose: Some(false),
    };

    let file_cfg = Config {
        mode: Some("debug"),
        threads: Some(8),
        path: Some("/etc/service"),
        verbose: None,
    };

    let env_cfg = Config {
        mode: None,
        threads: None,
        path: Some("/srv/app"),
        verbose: Some(true),
    };

    let cli_cfg = Config {
        mode: None,
        threads: None,
        path: Some(""),
        verbose: None,
    };

    let effective = merge(merge(merge(defaults, file_cfg), env_cfg), cli_cfg);

    println!("mode={}", effective.mode.unwrap_or("unset"));
    println!("threads={}", effective.threads.unwrap_or(0));
    println!("path={}", effective.path.unwrap_or("unset"));
    println!("verbose={}", effective.verbose.unwrap_or(false));
}
