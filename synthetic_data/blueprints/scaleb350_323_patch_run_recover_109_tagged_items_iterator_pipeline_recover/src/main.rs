fn main() {
    let input = [
        "Ada|active|backend, rust, ops",
        "Bob|inactive|backend,python",
        "Cy|active|frontend,ts",
        "Di|active|backend, rust , rust",
        "Eli|active|ops,backend,rust",
        "Fox|active|ops",
        "Gia|inactive|rust,ops",
        "Hal|active|backend",
        "Ivy|active|rust,frontend",
        "Jae|active|ops,backend",
    ];

    let mut rows: Vec<(String, usize, Vec<&str>)> = input
        .iter()
        .filter_map(|line| {
            let mut parts = line.split('|');
            let name = parts.next()?;
            let status = parts.next()?;
            let tags = parts.next()?;
            (status == "active").then_some((name, tags))
        })
        .map(|(name, tags)| {
            let kept: Vec<&str> = tags
                .split(',')
                .filter(|tag| matches!(*tag, "backend" | "rust" | "ops"))
                .collect();
            (name.to_string(), kept.len(), kept)
        })
        .filter(|(_, count, _)| *count >= 2)
        .collect();

    rows.sort_by(|a, b| a.0.cmp(&b.0));

    let mut out = Vec::new();
    for target in ["backend", "ops", "rust"] {
        let names: Vec<&str> = rows
            .iter()
            .filter(|(_, _, tags)| tags.contains(&target))
            .map(|(name, _, _)| name.as_str())
            .collect();
        out.push(format!("{}={}: {}", target, names.len(), names.join(", ")));
    }

    print!("{}", out.join("\n"));
}
