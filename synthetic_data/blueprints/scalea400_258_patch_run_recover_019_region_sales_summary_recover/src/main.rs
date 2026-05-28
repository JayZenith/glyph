use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Order {
    region: &'static str,
    amount: i32,
    refunded: bool,
}

fn main() {
    let orders = [
        Order { region: "North", amount: 120, refunded: false },
        Order { region: "South", amount: 80, refunded: true },
        Order { region: "North", amount: 20, refunded: false },
        Order { region: "East", amount: 90, refunded: false },
        Order { region: "East", amount: 75, refunded: false },
        Order { region: "West", amount: 40, refunded: false },
        Order { region: "West", amount: 95, refunded: false },
    ];

    let mut grouped: BTreeMap<&str, (i32, i32)> = BTreeMap::new();
    let mut total_orders = 0;
    let mut total_net = 0;

    for order in orders {
        let entry = grouped.entry(order.region).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += order.amount;
        total_orders += 1;
        total_net += order.amount;
    }

    let mut lines = Vec::new();
    for (region, (count, net)) in grouped {
        let avg = net / total_orders;
        lines.push(format!("{region}: orders={count} net={net} avg={avg}"));
    }
    lines.push(format!("TOTAL orders={} net={}", total_orders, total_net));

    print!("{}", lines.join("\n"));
}
