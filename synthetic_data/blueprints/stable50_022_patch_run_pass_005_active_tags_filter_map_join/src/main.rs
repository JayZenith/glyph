struct Tag {
    name: &'static str,
    active: bool,
}

fn active_labels(tags: &[Tag]) -> String {
    tags.iter()
        .filter(|tag| !tag.active)
        .map(|tag| tag.name.to_uppercase())
        .collect::<Vec<_>>()
        .join(",")
}

fn main() {
    let tags = [
        Tag { name: "rust", active: true },
        Tag { name: "old", active: false },
        Tag { name: "fast", active: true },
        Tag { name: "safe", active: true },
    ];

    println!("{}", active_labels(&tags));
}
