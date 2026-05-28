use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Order {
    region: &'static str,
    amount: u32,
    refunded: bool,
}

fn build_report(orders: &[Order]) -> String {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();

    for order in orders {
        *totals.entry(order.region).or_insert(0) += order.amount;
    }

    let mut rows: Vec<(&str, u32)> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    let grand_total: u32 = orders.iter().map(|o| o.amount).sum();

    let mut out = String::new();
    for (idx, (region, total)) in rows.iter().enumerate() {
        if idx > 0 {
            out.push('\n');
        }
        out.push_str(region);
        out.push_str(": ");
        out.push_str(&total.to_string());
    }
    out.push('\n');
    out.push_str("Grand Total: ");
    out.push_str(&grand_total.to_string());
    out
}

fn main() {
    let orders = [
        Order { region: "North", amount: 120, refunded: false },
        Order { region: "South", amount: 70, refunded: false },
        Order { region: "North", amount: 40, refunded: false },
        Order { region: "West", amount: 90, refunded: false },
        Order { region: "South", amount: 30, refunded: true },
        Order { region: "West", amount: 60, refunded: false },
    ];

    println!("{}", build_report(&orders));
}
