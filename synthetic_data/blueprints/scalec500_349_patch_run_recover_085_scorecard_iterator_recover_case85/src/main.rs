struct Item {
    name: &'static str,
    enabled: bool,
    score: i32,
}

fn main() {
    let items = vec![
        Item { name: "alpha", enabled: true, score: 12 },
        Item { name: "beta", enabled: false, score: 8 },
        Item { name: "gamma", enabled: true, score: 9 },
        Item { name: "delta", enabled: true, score: 0 },
        Item { name: "epsilon", enabled: true, score: 15 },
    ];

    let active: Vec<&Item> = items
        .iter()
        .filter(|item| item.enabled || item.score > 0)
        .collect();

    let names = active
        .iter()
        .map(|item| item.name.to_uppercase())
        .collect::<Vec<_>>()
        .join(", ");

    let total_score: i32 = active.iter().map(|item| item.score).sum();

    println!("active count: {}", active.len());
    println!("active names: {}", names);
    println!("total score: {}", total_score);
}
