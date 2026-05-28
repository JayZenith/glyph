fn main() {
    let entries = [
        ("ann", Some(8), true),
        ("bob", None, true),
        ("cy", Some(9), true),
        ("eve", Some(4), false),
        ("dee", Some(5), true),
    ];

    let kept: Vec<(String, i32)> = entries
        .into_iter()
        .filter(|(_, score, active)| *active && score.is_some())
        .map(|(name, score, _)| (name.to_string(), score.unwrap()))
        .filter(|(_, score)| *score >= 5)
        .collect();

    let labels = kept
        .iter()
        .map(|(name, score)| format!("{}:{}", name, score))
        .collect::<Vec<_>>()
        .join(", ");

    let total: i32 = kept.iter().map(|(_, score)| *score).sum();

    println!("{}", labels);
    println!("count={} total={}", kept.len(), total);
}
