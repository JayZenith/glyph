use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Sale {
    region: &'static str,
    units: u32,
    unit_price: u32,
    refunded: bool,
}

fn main() {
    let sales = [
        Sale { region: "North", units: 3, unit_price: 5, refunded: false },
        Sale { region: "South", units: 2, unit_price: 8, refunded: true },
        Sale { region: "North", units: 4, unit_price: 5, refunded: false },
        Sale { region: "South", units: 10, unit_price: 2, refunded: false },
        Sale { region: "West", units: 1, unit_price: 100, refunded: false },
        Sale { region: "South", units: 1, unit_price: 8, refunded: false },
    ];

    let mut by_region: BTreeMap<&str, (u32, u32, u32)> = BTreeMap::new();
    let mut total_orders = 0;
    let mut total_units = 0;
    let mut total_revenue = 0;

    for sale in sales {
        let entry = by_region.entry(sale.region).or_insert((0, 0, 0));
        entry.0 += 1;
        entry.1 += sale.units;
        entry.2 += sale.units * sale.unit_price;
        total_orders += 1;
        total_units += sale.units;
        total_revenue += sale.units * sale.unit_price;
    }

    let mut lines = Vec::new();
    for (region, (orders, units, revenue)) in by_region {
        let avg = revenue as f64 / orders as f64;
        lines.push(format!(
            "{}: orders={} units={} revenue={} avg={:.2}",
            region, orders, units, revenue, avg
        ));
    }
    lines.push(format!(
        "TOTAL: orders={} units={} revenue={}",
        total_orders, total_units, total_revenue
    ));

    println!("{}", lines.join("\n"));
}
