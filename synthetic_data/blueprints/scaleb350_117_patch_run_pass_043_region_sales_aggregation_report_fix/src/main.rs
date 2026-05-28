use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Sale {
    region: &'static str,
    units: u32,
    amount: f64,
    refunded: bool,
}

fn main() {
    let sales = [
        Sale { region: "North", units: 4, amount: 10.0, refunded: false },
        Sale { region: "South", units: 3, amount: 9.0, refunded: false },
        Sale { region: "North", units: 2, amount: 5.5, refunded: false },
        Sale { region: "South", units: 1, amount: 2.0, refunded: true },
        Sale { region: "West", units: 5, amount: 15.0, refunded: false },
        Sale { region: "South", units: 2, amount: 7.0, refunded: false },
    ];

    let mut per_region: BTreeMap<&str, (u32, u32, f64)> = BTreeMap::new();
    let mut total_orders = 0;
    let mut total_units = 0;
    let mut total_revenue = 0.0;

    for sale in sales {
        let entry = per_region.entry(sale.region).or_insert((0, 0, 0.0));
        entry.0 += 1;
        entry.1 += sale.units;
        entry.2 += sale.amount;
        total_orders += 1;
        total_units += sale.units;
        total_revenue += sale.amount;
    }

    let mut out = Vec::new();
    for (region, (orders, units, revenue)) in per_region {
        out.push(format!(
            "{}: orders={} units={} revenue={:.2}",
            region, orders, units, revenue
        ));
    }
    out.push(format!(
        "TOTAL: orders={} units={} revenue={:.2}",
        total_orders, total_units, total_revenue
    ));

    println!("{}", out.join("\n"));
}
