struct Task {
    name: &'static str,
    done: bool,
    tags: &'static [&'static str],
}

fn main() {
    let tasks = vec![
        Task { name: "plan", done: false, tags: &["score:5", "ops"] },
        Task { name: "deploy", done: false, tags: &["score:8", "release"] },
        Task { name: "retro", done: true, tags: &["score:3"] },
        Task { name: "docs", done: false, tags: &["score:x", "score:4"] },
        Task { name: "cleanup", done: false, tags: &["misc"] },
    ];

    let mut picked: Vec<(&str, i32)> = tasks
        .iter()
        .filter_map(|task| {
            task.tags
                .iter()
                .find_map(|tag| tag.strip_prefix("score:")?.parse::<i32>().ok())
                .map(|score| (task.name, score))
        })
        .collect();

    picked.sort_by_key(|(name, _)| *name);

    let names = picked.iter().map(|(name, _)| *name).collect::<Vec<_>>().join(", ");
    let total: i32 = picked.iter().map(|(_, score)| *score).sum();
    let avg = if picked.is_empty() {
        0.0
    } else {
        total as f64 / picked.len() as f64
    };

    println!("{}", names);
    println!("count={} total={} avg={:.1}", picked.len(), total, avg);
}
