fn main() {
    let words = ["pear", "plum", "fig", "kiwi", "apple"];

    let joined = words
        .iter()
        .filter_map(|word| (word.len() % 2 == 1).then(|| word.len().to_string()))
        .collect::<Vec<_>>()
        .join(",");

    println!("{}", joined);
}
