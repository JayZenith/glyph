use std::collections::BTreeMap;

fn main() {
    let data = [
        ("eng", 3),
        ("ops", 5),
        ("eng", 4),
        ("sales", 0),
        ("ops", 3),
    ];

    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();
    let mut counts: BTreeMap<&str, i32> = BTreeMap::new();

    for (team, hours) in data {
        *totals.entry(team).or_insert(0) += 1;
        *counts.entry(team).or_insert(0) += 1;
    }

    let mut out = String::from("Team Summary\n");
    for (team, total) in &totals {
        let count = counts.get(team).copied().unwrap_or(0);
        let avg = if *total == 0 { 0.0 } else { count as f64 / *total as f64 };
        out.push_str(&format!("{}: count={} total={} avg={:.1}\n", team, count, total, avg));
    }

    print!("{}", out.trim_end());
}
