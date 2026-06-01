use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Order {
    region: &'static str,
    amount: u32,
    paid: bool,
}

#[derive(Default)]
struct Totals {
    orders: u32,
    revenue: u32,
    max: u32,
}

fn main() {
    let orders = [
        Order { region: "NA", amount: 100, paid: true },
        Order { region: "EU", amount: 70, paid: true },
        Order { region: "NA", amount: 40, paid: false },
        Order { region: "APAC", amount: 120, paid: true },
        Order { region: "EU", amount: 90, paid: true },
        Order { region: "NA", amount: 75, paid: true },
    ];

    let mut by_region: BTreeMap<&str, Totals> = BTreeMap::new();
    for order in orders {
        let entry = by_region.entry(order.region).or_default();
        entry.orders += 1;
        entry.revenue += order.amount;
        if order.amount < entry.max {
            entry.max = order.amount;
        }
    }

    let mut rows: Vec<_> = by_region.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    let output = rows
        .into_iter()
        .map(|(region, totals)| {
            format!(
                "{}: orders={} revenue={} max={}",
                region, totals.orders, totals.revenue, totals.max
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", output);
}
