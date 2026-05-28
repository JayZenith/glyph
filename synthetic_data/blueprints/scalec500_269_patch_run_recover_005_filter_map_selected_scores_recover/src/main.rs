struct Item {
    name: &'static str,
    score: Option<i32>,
    enabled: bool,
}

fn main() {
    let items = [
        Item { name: "alpha", score: Some(4), enabled: true },
        Item { name: "beta", score: Some(7), enabled: true },
        Item { name: "gamma", score: None, enabled: true },
        Item { name: "delta", score: Some(5), enabled: true },
        Item { name: "epsilon", score: Some(8), enabled: false },
        Item { name: "zeta", score: Some(2), enabled: true },
    ];

    let selected: Vec<(&str, i32)> = items
        .iter()
        .filter_map(|item| item.score.map(|score| (item.name, score)))
        .filter(|(_, score)| *score >= 5)
        .collect();

    let total: i32 = selected.iter().map(|(_, score)| score).sum();
    let names = selected.iter().map(|(name, _)| *name).collect::<Vec<_>>().join(",");

    println!("selected={} total={} names={}", selected.len(), total, names);
}
