use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Order {
    month: &'static str,
    region: &'static str,
    cents: i32,
    paid: bool,
}

fn main() {
    let orders = [
        Order { month: "2024-01", region: "east", cents: 1500, paid: true },
        Order { month: "2024-01", region: "west", cents: 700, paid: true },
        Order { month: "2024-01", region: "west", cents: 1500, paid: true },
        Order { month: "2024-01", region: "north", cents: 999, paid: false },
        Order { month: "2024-02", region: "east", cents: 1950, paid: true },
        Order { month: "2024-02", region: "south", cents: 2000, paid: true },
        Order { month: "2024-02", region: "west", cents: 0, paid: true },
        Order { month: "2024-02", region: "east", cents: 500, paid: false },
    ];

    let mut monthly: BTreeMap<&str, BTreeMap<&str, i32>> = BTreeMap::new();
    for order in orders {
        let by_region = monthly.entry(order.month).or_default();
        *by_region.entry(order.region).or_default() += order.cents;
    }

    let mut out = String::new();
    for (month, regions) in monthly {
        out.push_str(month);
        out.push('\n');

        let mut pairs: Vec<_> = regions.into_iter().collect();
        pairs.sort_by(|a, b| a.0.cmp(b.0));

        for (region, cents) in pairs {
            out.push_str(&format!("- {}: ${}.{}\n", region, cents / 100, (cents % 100).abs()));
        }
    }

    print!("{}", out.trim_end());
}
