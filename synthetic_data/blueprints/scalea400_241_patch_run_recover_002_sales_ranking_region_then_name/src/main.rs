use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Sale {
    name: &'static str,
    region: &'static str,
    amount: u32,
}

fn main() {
    let sales = [
        Sale { name: "Alex", region: "East", amount: 8 },
        Sale { name: "Blake", region: "East", amount: 10 },
        Sale { name: "Alex", region: "West", amount: 13 },
        Sale { name: "Casey", region: "North", amount: 7 },
        Sale { name: "Blake", region: "East", amount: 3 },
        Sale { name: "Alex", region: "East", amount: 5 },
        Sale { name: "Drew", region: "South", amount: 12 },
        Sale { name: "Casey", region: "North", amount: 5 },
    ];

    let mut totals: BTreeMap<(&str, &str), u32> = BTreeMap::new();
    for s in sales {
        *totals.entry((s.name, s.region)).or_insert(0) += s.amount;
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.0.cmp(b.0.0));

    for (idx, ((name, region), total)) in rows.into_iter().take(4).enumerate() {
        println!("{}. {} | {} | {}", idx + 1, name, region, total);
    }
}
