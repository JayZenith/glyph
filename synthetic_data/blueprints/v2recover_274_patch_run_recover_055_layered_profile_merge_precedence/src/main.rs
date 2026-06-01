use std::fmt::Write;

#[derive(Clone, Debug)]
struct Config {
    region: Option<&'static str>,
    retries: Option<u32>,
    timeout: Option<u32>,
    verbose: Option<bool>,
}

impl Config {
    fn new() -> Self {
        Self {
            region: None,
            retries: None,
            timeout: None,
            verbose: None,
        }
    }

    fn merge(&mut self, other: &Config) {
        if self.region.is_none() {
            self.region = other.region;
        }
        if self.retries.is_none() {
            self.retries = other.retries;
        }
        if self.timeout.is_none() {
            self.timeout = other.timeout;
        }
        if self.verbose.is_none() {
            self.verbose = other.verbose;
        }
    }
}

fn main() {
    let defaults = Config {
        region: Some("us-east"),
        retries: Some(3),
        timeout: Some(30),
        verbose: Some(false),
    };

    let profile = Config {
        region: Some("eu-west"),
        retries: None,
        timeout: Some(20),
        verbose: Some(true),
    };

    let env = Config {
        region: None,
        retries: Some(5),
        timeout: None,
        verbose: None,
    };

    let mut effective = Config::new();
    effective.merge(&env);
    effective.merge(&profile);
    effective.merge(&defaults);

    let mut out = String::new();
    writeln!(&mut out, "region={}", effective.region.unwrap_or("unset")).unwrap();
    writeln!(&mut out, "retries={}", effective.retries.unwrap_or(0)).unwrap();
    writeln!(&mut out, "timeout={}", effective.timeout.unwrap_or(0)).unwrap();
    write!(&mut out, "verbose={}", effective.verbose.unwrap_or(false)).unwrap();
    print!("{}", out);
}
