fn main() {
    let rows = [
        ("Ava", Some(8), true),
        ("Ben", Some(3), true),
        ("Kai", None, true),
        ("Mia", Some(10), true),
        ("Ian", Some(6), false),
        ("Zoe", Some(4), true),
    ];

    let kept: Vec<(String, i32)> = rows
        .iter()
        .filter(|(_, score, active)| *active && score.is_some())
        .filter_map(|(name, score, _)| {
            score
                .filter(|n| n % 2 == 0)
                .map(|n| (format!("{}:{}", name, n), n))
        })
        .collect();

    let total: i32 = kept.iter().map(|(_, n)| *n).sum();

    for (label, _) in &kept {
        println!("{}", label);
    }
    println!("count={} total={}", kept.len(), total);
}
