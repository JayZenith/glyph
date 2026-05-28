use std::collections::BTreeMap;

struct Order {
    region: &'static str,
    amount: u32,
    shipped: bool,
}

fn main() {
    let orders = vec![
        Order { region: "North", amount: 120, shipped: true },
        Order { region: "South", amount: 80, shipped: false },
        Order { region: "North", amount: 50, shipped: false },
        Order { region: "West", amount: 40, shipped: true },
        Order { region: "South", amount: 30, shipped: true },
        Order { region: "North", amount: 0, shipped: true },
    ];

    let mut summary: BTreeMap<&str, (u32, u32, u32, u32)> = BTreeMap::new();
    for order in orders {
        let entry = summary.entry(order.region).or_insert((0, 0, 0, 0));
        entry.0 += 1;
        if order.amount > 0 {
            entry.3 += order.amount;
        }
        if order.shipped {
            entry.1 += 1;
        }
    }

    for (region, (orders, shipped, pending, revenue)) in summary {
        println!(
            "{region}: orders={orders} shipped={shipped} pending={pending} revenue={revenue}"
        );
    }
}
