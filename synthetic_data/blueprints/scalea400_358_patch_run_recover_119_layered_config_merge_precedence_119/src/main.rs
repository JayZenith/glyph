#[derive(Clone, Debug)]
struct Layer {
    mode: Option<&'static str>,
    threads: Option<u32>,
    timeout: Option<u32>,
    plugins: Vec<&'static str>,
    disable_plugins: bool,
}

#[derive(Debug)]
struct Effective {
    mode: &'static str,
    threads: u32,
    timeout: u32,
    plugins: Vec<&'static str>,
}

fn pick_mode(defaults: &Layer, env: &Layer, user: &Layer) -> &'static str {
    user.mode.or(env.mode).or(defaults.mode).unwrap_or("safe")
}

fn merge_plugins(defaults: &Layer, env: &Layer, user: &Layer) -> Vec<&'static str> {
    let mut out = Vec::new();
    for p in defaults.plugins.iter().chain(env.plugins.iter()).chain(user.plugins.iter()) {
        if !out.contains(p) {
            out.push(*p);
        }
    }
    out
}

fn merge(defaults: &Layer, env: &Layer, user: &Layer) -> Effective {
    Effective {
        mode: pick_mode(defaults, env, user),
        threads: user.threads.or(defaults.threads).unwrap_or(4),
        timeout: user.timeout.or(env.timeout).or(defaults.timeout).unwrap_or(30),
        plugins: merge_plugins(defaults, env, user),
    }
}

fn main() {
    let defaults = Layer {
        mode: Some("safe"),
        threads: Some(4),
        timeout: Some(30),
        plugins: vec!["core", "metrics"],
        disable_plugins: false,
    };

    let env = Layer {
        mode: Some("manual"),
        threads: Some(8),
        timeout: Some(20),
        plugins: vec!["trace", "metrics"],
        disable_plugins: false,
    };

    let user = Layer {
        mode: Some("inherit"),
        threads: None,
        timeout: Some(10),
        plugins: vec!["debug", "trace"],
        disable_plugins: true,
    };

    let eff = merge(&defaults, &env, &user);
    println!("mode={}", eff.mode);
    println!("threads={}", eff.threads);
    println!("timeout={}", eff.timeout);
    println!("plugins={:?}", eff.plugins);
}
