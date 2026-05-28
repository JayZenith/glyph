use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy)]
struct Order {
    month: &'static str,
    region: &'static str,
    customer: &'static str,
    status: &'static str,
    amount: u32,
}

fn main() {
    let orders = [
        Order { month: "2024-01", region: "north", customer: "acme", status: "shipped", amount: 100 },
        Order { month: "2024-01", region: "north", customer: "acme", status: "shipped", amount: 50 },
        Order { month: "2024-01", region: "north", customer: "brio", status: "cancelled", amount: 40 },
        Order { month: "2024-01", region: "south", customer: "brio", status: "shipped", amount: 80 },
        Order { month: "2024-01", region: "south", customer: "coda", status: "shipped", amount: 75 },
        Order { month: "2024-01", region: "west", customer: "dune", status: "pending", amount: 20 },
        Order { month: "2024-01", region: "west", customer: "dune", status: "shipped", amount: 90 },
        Order { month: "2024-02", region: "north", customer: "acme", status: "shipped", amount: 120 },
        Order { month: "2024-02", region: "south", customer: "coda", status: "shipped", amount: 60 },
        Order { month: "2024-02", region: "south", customer: "echo", status: "returned", amount: 30 },
        Order { month: "2024-02", region: "south", customer: "echo", status: "shipped", amount: 90 },
        Order { month: "2024-02", region: "west", customer: "foxtrot", status: "shipped", amount: 70 },
    ];

    let mut months: BTreeMap<&str, BTreeMap<&str, (u32, BTreeSet<&str>, u32)>> = BTreeMap::new();

    for o in orders {
        let region_map = months.entry(o.month).or_default();
        let entry = region_map.entry(o.region).or_insert((0, BTreeSet::new(), 0));
        entry.0 += 1;
        entry.1.insert(o.customer);
        entry.2 += o.amount;
    }

    let mut lines = Vec::new();
    for (month, region_map) in months {
        lines.push(month.to_string());
        let mut rows: Vec<_> = region_map.into_iter().collect();
        rows.sort_by(|a, b| a.0.cmp(b.0));
        for (region, (orders, customers, revenue)) in rows {
            lines.push(format!(
                "{} | orders={} | customers={} | revenue={}",
                region,
                orders,
                customers.len(),
                revenue
            ));
        }
    }

    print!("{}", lines.join("\n"));
}
