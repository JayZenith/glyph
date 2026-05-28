fn leaderboard(rows: &[(&str, i32)]) -> String {
    let mut best: Vec<(&str, i32)> = Vec::new();
    for &(name, score) in rows {
        if let Some((_, saved)) = best.iter_mut().find(|(n, _)| *n == name) {
            if score > *saved {
                *saved = score;
            }
        } else {
            best.push((name, score));
        }
    }

    best.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(b.0)));

    best.into_iter()
        .enumerate()
        .map(|(i, (name, score))| format!("{}. {} {}", i + 1, name, score))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let rows = [
        ("Ava", 10),
        ("Bea", 12),
        ("Cara", 9),
        ("Bea", 8),
        ("Cara", 12),
        ("Dan", 11),
    ];

    println!("{}", leaderboard(&rows));
}
