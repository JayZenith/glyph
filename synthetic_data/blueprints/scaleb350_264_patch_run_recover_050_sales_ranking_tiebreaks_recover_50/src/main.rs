use std::collections::BTreeMap;

#[derive(Debug, Clone)]
struct Entry {
    name: &'static str,
    total: i32,
    attempts: usize,
}

fn main() {
    let rounds = [
        ("Ava", 10),
        ("Bea", 5),
        ("Ava", 5),
        ("Mia", 7),
        ("Bea", 7),
        ("Eli", 8),
        ("Mia", 3),
        ("Ian", 9),
        ("Eli", 7),
        ("Zoe", 12),
        ("Mia", 5),
    ];

    let mut totals: BTreeMap<&'static str, (i32, usize)> = BTreeMap::new();
    for (name, points) in rounds {
        let entry = totals.entry(name).or_insert((0, 0));
        entry.0 += points;
        entry.1 += 1;
    }

    let mut rows: Vec<Entry> = totals
        .into_iter()
        .map(|(name, (total, attempts))| Entry {
            name,
            total,
            attempts,
        })
        .collect();

    rows.sort_by(|a, b| {
        b.total
            .cmp(&a.total)
            .then_with(|| a.name.cmp(b.name))
            .then_with(|| a.attempts.cmp(&b.attempts))
    });

    for (idx, row) in rows.iter().enumerate() {
        println!(
            "{}. {} => {} pts ({} attempts)",
            idx + 1,
            row.name,
            row.total,
            row.attempts
        );
    }
}
