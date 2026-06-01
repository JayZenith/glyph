fn main() {
    let rows = vec![
        ("Rust", 10),
        ("Go", 9),
        ("Python", 8),
        ("Rust", 3),
        ("Java", 9),
        ("C", 7),
        ("Ruby", 7),
        ("Elixir", 7),
    ];

    let mut items = rows;
    items.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

    for (idx, (name, solved)) in items.iter().enumerate().take(5) {
        println!("{}. {} - {}", idx + 1, name, solved);
    }
}
