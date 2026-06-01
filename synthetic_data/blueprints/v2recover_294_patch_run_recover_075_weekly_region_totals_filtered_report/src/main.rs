use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Order {
    region: &'static str,
    units: u32,
    priority: bool,
    cancelled: bool,
}

fn main() {
    let orders = [
        Order { region: "north", units: 4, priority: false, cancelled: false },
        Order { region: "south", units: 3, priority: true, cancelled: false },
        Order { region: "north", units: 6, priority: true, cancelled: false },
        Order { region: "east", units: 7, priority: true, cancelled: false },
        Order { region: "west", units: 2, priority: false, cancelled: false },
        Order { region: "west", units: 9, priority: true, cancelled: false },
        Order { region: "south", units: 8, priority: false, cancelled: true },
    ];

    let mut shipped: BTreeMap<&str, u32> = BTreeMap::new();
    let mut priority_counts: BTreeMap<&str, u32> = BTreeMap::new();

    for order in orders {
        *shipped.entry(order.region).or_insert(0) += order.units;
        if order.priority {
            *priority_counts.entry(order.region).or_insert(0) += order.units;
        }
    }

    let mut rows: Vec<_> = shipped.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    let mut out = String::from("Weekly regional totals\n");
    for (region, units) in rows {
        if units >= 3 {
            let p = priority_counts.get(region).copied().unwrap_or(0);
            out.push_str(&format!("{} shipped={} priority={}\n", region, units, p));
        }
    }

    print!("{}", out.trim_end());
}
