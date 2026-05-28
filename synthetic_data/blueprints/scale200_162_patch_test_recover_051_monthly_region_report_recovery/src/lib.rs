use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Order {
    pub region: &'static str,
    pub amount_cents: u32,
    pub paid: bool,
    pub refunded: bool,
}

pub fn render_region_report(orders: &[Order]) -> String {
    let mut totals: BTreeMap<&str, (u32, u32)> = BTreeMap::new();

    for order in orders {
        if !order.paid {
            continue;
        }
        let entry = totals.entry(order.region).or_insert((0, 0));
        entry.0 += order.amount_cents;
        entry.1 += 1;
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    let mut out = String::from("region | revenue | orders\n");
    for (region, (amount, _count)) in rows {
        out.push_str(&format!("{} | ${}.{:02} | 1\n", region, amount / 100, amount % 100));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_orders() -> Vec<Order> {
        vec![
            Order { region: "west", amount_cents: 1200, paid: true, refunded: false },
            Order { region: "east", amount_cents: 900, paid: true, refunded: false },
            Order { region: "west", amount_cents: 800, paid: true, refunded: true },
            Order { region: "north", amount_cents: 500, paid: false, refunded: false },
            Order { region: "east", amount_cents: 700, paid: true, refunded: false },
            Order { region: "south", amount_cents: 1600, paid: true, refunded: false },
            Order { region: "south", amount_cents: 400, paid: true, refunded: false },
        ]
    }

    #[test]
    fn excludes_unpaid_and_refunded_orders() {
        let report = render_region_report(&sample_orders());
        assert!(!report.contains("north"));
        assert!(!report.contains("$20.00 | 2\nwest"));
        assert!(report.contains("west | $12.00 | 1"));
    }

    #[test]
    fn sorts_by_revenue_desc_then_region_name() {
        let report = render_region_report(&sample_orders());
        let body: Vec<&str> = report.lines().skip(1).collect();
        assert_eq!(body, vec![
            "south | $20.00 | 2",
            "east | $16.00 | 2",
            "west | $12.00 | 1",
        ]);
    }

    #[test]
    fn ties_use_region_name_ascending() {
        let orders = vec![
            Order { region: "beta", amount_cents: 500, paid: true, refunded: false },
            Order { region: "alpha", amount_cents: 500, paid: true, refunded: false },
        ];
        let report = render_region_report(&orders);
        let body: Vec<&str> = report.lines().skip(1).collect();
        assert_eq!(body, vec![
            "alpha | $5.00 | 1",
            "beta | $5.00 | 1",
        ]);
    }
}
