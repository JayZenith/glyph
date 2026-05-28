use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub region: &'static str,
    pub amount_cents: i64,
    pub refunded: bool,
}

pub fn region_report(orders: &[Order]) -> String {
    let mut totals: BTreeMap<&str, (usize, i64)> = BTreeMap::new();

    for order in orders {
        let entry = totals.entry(order.region).or_insert((0, 0));
        entry.0 += 1;
        if !order.refunded {
            entry.1 += order.amount_cents;
        }
    }

    let mut out = String::new();
    let mut grand_orders = 0usize;
    let mut grand_revenue = 0i64;

    for (region, (count, revenue)) in totals {
        grand_orders += count;
        grand_revenue += revenue;
        out.push_str(&format!("{}: {} orders, ${:.2}\n", region, count, revenue as f64 / 100.0));
    }

    out.push_str(&format!(
        "TOTAL: {} orders, ${:.2}",
        grand_orders,
        grand_revenue as f64 / 100.0
    ));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_sorted_regions_and_excludes_refunds_from_counts_and_revenue() {
        let orders = vec![
            Order { region: "west", amount_cents: 1200, refunded: false },
            Order { region: "east", amount_cents: 500, refunded: true },
            Order { region: "east", amount_cents: 700, refunded: false },
            Order { region: "west", amount_cents: 300, refunded: true },
            Order { region: "north", amount_cents: 250, refunded: false },
        ];

        let got = region_report(&orders);
        let expected = concat!(
            "east: 1 orders, $7.00\n",
            "north: 1 orders, $2.50\n",
            "west: 1 orders, $12.00\n",
            "TOTAL: 3 orders, $21.50"
        );

        assert_eq!(got, expected);
    }

    #[test]
    fn includes_regions_with_only_refunds_as_zero_lines() {
        let orders = vec![
            Order { region: "central", amount_cents: 900, refunded: true },
            Order { region: "south", amount_cents: 100, refunded: false },
        ];

        let got = region_report(&orders);
        let expected = concat!(
            "central: 0 orders, $0.00\n",
            "south: 1 orders, $1.00\n",
            "TOTAL: 1 orders, $1.00"
        );

        assert_eq!(got, expected);
    }
}
