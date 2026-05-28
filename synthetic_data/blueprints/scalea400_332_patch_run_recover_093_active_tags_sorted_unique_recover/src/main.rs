fn active_tags(records: &[(&str, bool)]) -> String {
    let mut tags: Vec<&str> = records
        .iter()
        .filter(|(_, active)| *active)
        .map(|(tag, _)| *tag)
        .collect();
    tags.sort();
    tags.join(", ")
}

fn main() {
    let records = [
        ("blue", true),
        ("red", false),
        ("green", true),
        ("blue", true),
        ("red", true),
        ("amber", false),
    ];

    println!("active tags: {}", active_tags(&records));
}
