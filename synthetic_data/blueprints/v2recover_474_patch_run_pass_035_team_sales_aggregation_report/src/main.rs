use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Sale {
    region: &'static str,
    units: u32,
    revenue: u32,
    refunded: bool,
}

fn main() {
    let sales = [
        Sale { region: "East", units: 3, revenue: 45, refunded: false },
        Sale { region: "West", units: 2, revenue: 40, refunded: true },
        Sale { region: "East", units: 2, revenue: 40, refunded: false },
        Sale { region: "North", units: 5, revenue: 50, refunded: false },
        Sale { region: "West", units: 3, revenue: 60, refunded: false },
        Sale { region: "West", units: 4, revenue: 40, refunded: false },
    ];

    let mut grouped: BTreeMap<&str, (u32, u32, u32)> = BTreeMap::new();
    for sale in sales {
        let entry = grouped.entry(sale.region).or_insert((0, 0, 0));
        entry.0 += 1;
        entry.1 += sale.units;
        entry.2 += sale.revenue;
    }

    let mut out = String::from("Region Summary\n");
    let mut top_region = "";
    let mut top_revenue = 0;

    for (region, (orders, units, revenue)) in grouped {
        let avg = revenue as f64 / units as f64;
        out.push_str(&format!(
            "{}: orders={} units={} revenue={} avg={:.2}\n",
            region, orders, units, revenue, avg
        ));
        if revenue >= top_revenue {
            top_region = region;
            top_revenue = revenue;
        }
    }

    out.push_str(&format!("Top region: {} ({})", top_region, top_revenue));
    print!("{}", out);
}
