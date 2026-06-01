fn main() {
    let tags = [
        Some("rust"),
        None,
        Some("fast"),
        Some(""),
        Some("safe"),
        None,
        Some("stable"),
    ];

    let summary = tags
        .iter()
        .filter_map(|tag| match tag {
            Some(value) if value.is_empty() => Some(*value),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(",");

    print!("{}", summary);
}
