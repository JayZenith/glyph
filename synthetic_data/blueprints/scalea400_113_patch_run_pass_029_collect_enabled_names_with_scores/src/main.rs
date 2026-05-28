struct Item {
    name: &'static str,
    enabled: bool,
    values: &'static [i32],
}

fn main() {
    let items = [
        Item { name: "alpha", enabled: true, values: &[1, 2, 3] },
        Item { name: "beta", enabled: false, values: &[4, 1] },
        Item { name: "gamma", enabled: true, values: &[5, 3] },
    ];

    let lines: Vec<String> = items
        .iter()
        .filter(|item| !item.enabled)
        .map(|item| format!("{}:{}", item.name.to_uppercase(), item.values.iter().max().copied().unwrap_or(0)))
        .collect();

    println!("{}", lines.join("\n"));
}
