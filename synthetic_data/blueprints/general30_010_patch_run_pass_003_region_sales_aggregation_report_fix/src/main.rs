use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Sale {
    region: &'static str,
    units: u32,
    unit_price_cents: u32,
    refunded: bool,
}

#[derive(Default)]
struct Totals {
    orders: u32,
    units: u32,
    revenue_cents: u32,
}

fn main() {
    let sales = [
        Sale { region: "North", units: 3, unit_price_cents: 250, refunded: false },
        Sale { region: "South", units: 2, unit_price_cents: 400, refunded: false },
        Sale { region: "North", units: 1, unit_price_cents: 500, refunded: true },
        Sale { region: "South", units: 4, unit_price_cents: 175, refunded: false },
        Sale { region: "North", units: 2, unit_price_cents: 275, refunded: false },
    ];

    let mut by_region: BTreeMap<&str, Totals> = BTreeMap::new();
    let mut grand = Totals::default();

    for sale in sales {
        let entry = by_region.entry(sale.region).or_default();
        entry.orders += 1;
        entry.units += sale.units;
        entry.revenue_cents += sale.units * sale.unit_price_cents;

        grand.orders += 1;
        grand.units += sale.units;
        grand.revenue_cents += sale.units * sale.unit_price_cents;
    }

    let mut lines = Vec::new();
    for (region, total) in by_region {
        lines.push(format!(
            "{} | orders={} | units={} | revenue={:.2}",
            region,
            total.orders,
            total.units,
            total.revenue_cents as f64 / 100.0
        ));
    }
    lines.push(format!(
        "TOTAL | orders={} | units={} | revenue={:.2}",
        grand.orders,
        grand.units,
        grand.revenue_cents as f64 / 100.0
    ));

    println!("{}", lines.join("\n"));
}
