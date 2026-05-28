fn main() {
    let mut players = vec![
        ("Zoe", 17),
        ("Ava", 9),
        ("Mia", 17),
        ("Ben", 12),
        ("Cara", 17),
        ("Eli", 12),
    ];

    players.sort_by(|a, b| a.0.cmp(b.0));

    for (i, (name, score)) in players.iter().enumerate() {
        println!("{}. {} ({})", i + 1, name, score);
    }
}
