use std::collections::BTreeMap;

struct Order {
    region: &'static str,
    amount: u32,
    shipped: bool,
    cancelled: bool,
}

fn main() {
    let orders = vec![
        Order { region: "North", amount: 120, shipped: true, cancelled: false },
        Order { region: "south", amount: 80, shipped: true, cancelled: false },
        Order { region: "NORTH", amount: 70, shipped: true, cancelled: false },
        Order { region: "West", amount: 100, shipped: true, cancelled: false },
        Order { region: "South", amount: 30, shipped: false, cancelled: false },
        Order { region: "north", amount: 50, shipped: true, cancelled: true },
    ];

    let mut totals: BTreeMap<String, u32> = BTreeMap::new();
    for order in orders {
        if !order.cancelled {
            *totals.entry(order.region.to_string()).or_insert(0) += order.amount;
        }
    }

    let mut lines: Vec<(String, u32)> = totals.into_iter().collect();
    lines.sort_by(|a, b| a.0.cmp(&b.0));

    println!("Sales report");
    for (region, total) in lines {
        println!("{}: {}", region, total);
    }
}
