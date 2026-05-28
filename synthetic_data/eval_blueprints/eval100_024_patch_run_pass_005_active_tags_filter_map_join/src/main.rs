fn main() {
    let items = [
        ("core", true),
        ("draft", false),
        ("fast", true),
        ("wip", false),
        ("safe", true),
    ];

    let output = items
        .iter()
        .filter_map(|(name, enabled)| if !enabled { Some(*name) } else { None })
        .collect::<Vec<_>>()
        .join(",");

    println!("{}", output);
}
