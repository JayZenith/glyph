fn main() {
    let rows = [
        "r1|active|red, blue, red",
        "r2|active|green, blue, green, yellow",
        "r3|archived|blue, blue",
        "r4|active|",
        "r5|paused|orange",
    ];

    let mut items: Vec<(String, usize)> = rows
        .iter()
        .filter_map(|line| {
            let mut parts = line.split('|');
            let id = parts.next()?.to_string();
            let status = parts.next()?;
            let tags = parts.next()?;

            if status != "active" {
                return None;
            }

            let count = tags
                .split(',')
                .filter(|t| !t.is_empty())
                .count();

            Some((id, count))
        })
        .collect();

    items.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

    let out = items
        .into_iter()
        .map(|(id, count)| format!("{}:{}", id, count))
        .collect::<Vec<_>>()
        .join("\n");

    print!("{}", out);
}
