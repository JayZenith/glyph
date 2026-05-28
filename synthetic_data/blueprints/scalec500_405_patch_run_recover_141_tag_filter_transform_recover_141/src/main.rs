fn main() {
    let tags = [
        "core:fast",
        "ui:slow",
        " util:safe ",
        "db:",
        "data:hot",
        "misc",
        "cli:fast",
    ];

    let out = tags
        .iter()
        .filter_map(|raw| {
            let trimmed = raw.trim();
            let (name, level) = trimmed.split_once(':')?;
            if level.is_empty() {
                return None;
            }
            if level == "fast" || level == "hot" {
                Some(format!("{}:{}", name.to_uppercase(), level))
            } else {
                Some(format!("{}:{}", name, level.to_uppercase()))
            }
        })
        .collect::<Vec<_>>()
        .join(", ");

    println!("{out}");
}
