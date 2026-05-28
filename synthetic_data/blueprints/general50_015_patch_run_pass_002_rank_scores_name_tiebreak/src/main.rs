fn main() {
    let entries = vec![
        ("Bea", 12),
        ("Amy", 9),
        ("Cara", 10),
        ("Amy", 12),
        ("Dan", 11),
        ("Bea", 8),
    ];

    let mut best: Vec<(&str, i32)> = Vec::new();
    for (name, score) in entries {
        if let Some((_, s)) = best.iter_mut().find(|(n, _)| *n == name) {
            if score > *s {
                *s = score;
            }
        } else {
            best.push((name, score));
        }
    }

    best.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(b.0)));

    for (i, (name, score)) in best.iter().enumerate() {
        println!("{}. {} - {}", i + 1, name, score);
    }
}
