use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Order {
    region: &'static str,
    amount: i32,
    shipped: bool,
}

#[derive(Default)]
struct Summary {
    orders: usize,
    shipped: usize,
    net: i32,
}

fn main() {
    let orders = [
        Order { region: "North", amount: 120, shipped: true },
        Order { region: "North", amount: -20, shipped: false },
        Order { region: "South", amount: -5, shipped: false },
        Order { region: "East", amount: 0, shipped: true },
        Order { region: "West", amount: 30, shipped: true },
        Order { region: "West", amount: 50, shipped: false },
        Order { region: "West", amount: -15, shipped: true },
    ];

    let mut by_region: BTreeMap<&str, Summary> = BTreeMap::new();
    for order in orders {
        let entry = by_region.entry(order.region).or_default();
        entry.orders += 1;
        if order.shipped {
            entry.shipped += 1;
        }
        entry.net += order.amount;
    }

    let mut rows: Vec<_> = by_region.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    let mut out = Vec::new();
    for (region, summary) in rows {
        let avg = summary.net as f64 / summary.shipped.max(1) as f64;
        out.push(format!(
            "{} | orders={} | shipped={} | net={} | avg={:.2}",
            region, summary.orders, summary.shipped, summary.net, avg
        ));
    }

    print!("{}", out.join("\n"));
}
