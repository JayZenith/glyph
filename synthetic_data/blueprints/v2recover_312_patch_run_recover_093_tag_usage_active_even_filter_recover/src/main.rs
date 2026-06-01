use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Record {
    active: bool,
    tags: &'static [&'static str],
}

fn main() {
    let records = [
        Record {
            active: true,
            tags: &["red", " blue", "", "blue"],
        },
        Record {
            active: false,
            tags: &["blue", "green", "red"],
        },
        Record {
            active: true,
            tags: &["yellow", "blue ", "yellow"],
        },
        Record {
            active: true,
            tags: &["green", "red", "  ", "blue"],
        },
    ];

    let mut counts = BTreeMap::new();
    for tag in records.iter().flat_map(|r| r.tags.iter()) {
        *counts.entry(*tag).or_insert(0usize) += 1;
    }

    let mut rows: Vec<_> = counts.into_iter().collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));

    let output = rows
        .into_iter()
        .filter(|(_, count)| count % 2 == 1)
        .map(|(tag, count)| format!("{}:{}", tag, count))
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", output);
}
