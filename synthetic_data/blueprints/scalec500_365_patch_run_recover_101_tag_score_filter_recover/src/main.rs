fn tag_totals(lines: &[&str]) -> Vec<(String, i32)> {
    let mut totals = std::collections::BTreeMap::<String, i32>::new();

    lines
        .iter()
        .filter_map(|line| line.split_once(':'))
        .filter_map(|(tag, value)| value.parse::<i32>().ok().map(|n| (tag, n)))
        .filter(|(_, n)| *n >= 0)
        .for_each(|(tag, n)| {
            *totals.entry(tag.to_string()).or_insert(0) += n;
        });

    let mut items: Vec<_> = totals.into_iter().collect();
    items.sort_by(|a, b| a.0.cmp(&b.0));
    items
}

fn main() {
    let lines = [
        "rust:8",
        "cli:5",
        "docs:3",
        "rust:6",
        "skip",
        "cli:4",
        "docs:-2",
        "ops:0",
        "docs:5",
        "rust:0",
        "cli:1",
        "bad:x",
    ];

    let out = tag_totals(&lines)
        .into_iter()
        .filter(|(_, total)| *total > 0)
        .map(|(tag, total)| format!("{tag}={total}"))
        .collect::<Vec<_>>()
        .join("\n");

    println!("{out}");
}
