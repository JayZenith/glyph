use std::collections::BTreeMap;

#[derive(Default, Clone, Copy)]
struct Stats {
    total: i32,
    orders: u32,
}

fn main() {
    let data = [
        ("North", 100),
        ("South", 90),
        ("East", 70),
        ("West", 50),
        ("Central", 40),
        ("North", 60),
        ("South", 60),
        ("East", 90),
        ("West", 60),
        ("Central", 60),
        ("West", 50),
        ("Central", 50),
        ("South", -20),
    ];

    let mut by_region: BTreeMap<&str, Stats> = BTreeMap::new();
    for (region, amount) in data {
        let entry = by_region.entry(region).or_default();
        entry.total += amount;
        entry.orders += 1;
    }

    let mut rows: Vec<(&str, Stats)> = by_region.into_iter().collect();
    rows.sort_by(|a, b| {
        b.1.total
            .cmp(&a.1.total)
            .then_with(|| a.0.cmp(b.0))
    });

    for (idx, (region, stats)) in rows.iter().enumerate() {
        println!("{}. {} | total={} | orders={}", idx + 1, region, stats.total, stats.orders);
    }
}
