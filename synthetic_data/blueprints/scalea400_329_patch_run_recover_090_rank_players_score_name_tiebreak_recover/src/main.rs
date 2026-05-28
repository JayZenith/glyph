fn main() {
    let mut players = vec![
        ("Amy", 8),
        ("Bob", 10),
        ("Cara", 10),
        ("Dan", 9),
        ("Eve", 8),
    ];

    players.sort_by(|a, b| a.1.cmp(&b.1));

    let mut out = Vec::new();
    for (i, (name, score)) in players.iter().enumerate() {
        out.push(format!("{}. {} ({})", i + 1, name, score));
    }

    print!("{}", out.join("\n"));
}
