fn main() {
    let records = [
        ("ada", Some(4)),
        ("bob", Some(2)),
        ("eve", None),
        ("max", Some(5)),
        ("zoe", Some(1)),
        ("cy", Some(8)),
    ];

    let out = records
        .iter()
        .filter_map(|(name, score)| score.map(|s| format!("{}:{}", name, s)))
        .filter(|entry| {
            entry
                .split_once(':')
                .and_then(|(_, n)| n.parse::<u32>().ok())
                .map(|n| n % 2 == 1)
                .unwrap_or(false)
        })
        .collect::<Vec<_>>()
        .join(",");

    print!("{}", out);
}
