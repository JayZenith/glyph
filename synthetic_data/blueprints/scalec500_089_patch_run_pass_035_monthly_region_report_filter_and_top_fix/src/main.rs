use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Sale {
    region: &'static str,
    item: &'static str,
    amount: u32,
    refunded: bool,
}

fn main() {
    let sales = [
        Sale { region: "North", item: "tea", amount: 12, refunded: false },
        Sale { region: "North", item: "coffee", amount: 7, refunded: false },
        Sale { region: "South", item: "coffee", amount: 20, refunded: false },
        Sale { region: "South", item: "tea", amount: 5, refunded: false },
        Sale { region: "South", item: "coffee", amount: 9, refunded: true },
        Sale { region: "West", item: "tea", amount: 8, refunded: false },
    ];

    let mut by_region: BTreeMap<&str, (u32, u32, BTreeMap<&str, u32>)> = BTreeMap::new();

    for sale in sales {
        let entry = by_region
            .entry(sale.region)
            .or_insert((0, 0, BTreeMap::new()));
        entry.0 += 1;
        entry.1 += sale.amount;
        *entry.2.entry(sale.item).or_insert(0) += sale.amount;
    }

    let mut lines = Vec::new();
    for (region, (orders, total, items)) in by_region {
        let top = items
            .into_iter()
            .max_by_key(|(_, amount)| *amount)
            .map(|(item, _)| item)
            .unwrap_or("none");
        lines.push(format!("{}: orders={} total={} top={}", region, orders, total, top));
    }

    println!("{}", lines.join("\n"));
}
