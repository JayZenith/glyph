use std::collections::BTreeMap;

fn main() {
    let entries = [
        ("Sales", 120),
        ("Engineering", 200),
        ("HR", 80),
        ("Sales", 90),
        ("Engineering", 150),
        ("HR", 60),
        ("Engineering", 230),
        ("Sales", 60),
    ];

    let mut totals: BTreeMap<&str, (u32, i32)> = BTreeMap::new();
    for (dept, amount) in entries {
        let slot = totals.entry(dept).or_insert((0, 0));
        slot.0 += 1;
        slot.1 += amount;
    }

    let mut grand_total = 0;
    for (dept, (count, total)) in &totals {
        grand_total += total;
        println!("{}: count={} total={}", dept, count, total);
    }
    println!("Grand total: {}", grand_total);
}
