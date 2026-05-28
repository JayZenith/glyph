use std::collections::BTreeMap;

fn main() {
    let rows = [
        ("North", 12, true),
        ("South", 7, true),
        ("North", 5, true),
        ("East", 8, false),
        ("South", 3, true),
        ("West", 9, true),
        ("East", 4, false),
    ];

    let mut totals: BTreeMap<&str, (usize, i32)> = BTreeMap::new();
    for (region, amount, active) in rows {
        let entry = totals.entry(region).or_insert((0, 0));
        entry.0 += 1;
        if !active {
            entry.1 += amount;
        }
    }

    let mut lines = Vec::new();
    let mut grand_total = 0;
    for (region, (count, total)) in totals {
        grand_total += total;
        lines.push(format!("{}: count={} total={}", region, count, total));
    }

    lines.sort();
    lines.push(format!("Grand total: {}", grand_total));
    println!("{}", lines.join("\n"));
}
