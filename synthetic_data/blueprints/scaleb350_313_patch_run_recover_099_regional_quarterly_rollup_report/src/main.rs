use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Order {
    region: &'static str,
    quarter: u8,
    amount: u32,
    shipped: bool,
}

#[derive(Default)]
struct Totals {
    orders: u32,
    gross: u32,
    shipped: u32,
    pending: u32,
}

fn main() {
    let orders = [
        Order { region: "North", quarter: 2, amount: 120, shipped: true },
        Order { region: "North", quarter: 2, amount: 80, shipped: false },
        Order { region: "South", quarter: 2, amount: 200, shipped: true },
        Order { region: "South", quarter: 1, amount: 50, shipped: false },
        Order { region: "West", quarter: 2, amount: 30, shipped: false },
        Order { region: "West", quarter: 2, amount: 70, shipped: false },
        Order { region: "North", quarter: 2, amount: 40, shipped: true },
        Order { region: "South", quarter: 2, amount: 50, shipped: true },
    ];

    let mut by_region: BTreeMap<&str, Totals> = BTreeMap::new();
    let mut grand = Totals::default();

    for order in orders {
        if order.quarter != 2 {
            continue;
        }
        let entry = by_region.entry(order.region).or_default();
        entry.orders += 1;
        if order.shipped {
            entry.shipped += 1;
            entry.gross += order.amount;
        } else {
            entry.pending += 1;
        }

        grand.orders += 1;
        grand.gross += order.amount;
        if order.shipped {
            grand.shipped += 1;
        } else {
            grand.pending += 1;
        }
    }

    let mut lines = Vec::new();
    for (region, t) in by_region {
        let avg = if t.shipped == 0 { 0 } else { t.gross / t.shipped };
        lines.push(format!(
            "{} | orders={} | gross={} | shipped={} | pending={} | avg={}",
            region, t.orders, t.gross, t.shipped, t.pending, avg
        ));
    }
    lines.push(format!(
        "TOTAL | orders={} | gross={} | shipped={} | pending={}",
        grand.orders, grand.gross, grand.shipped, grand.pending
    ));

    print!("{}", lines.join("\n"));
}
