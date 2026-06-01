use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order<'a> {
    pub month: &'a str,
    pub region: &'a str,
    pub amount_cents: i64,
    pub refunded: bool,
}

pub fn monthly_region_report(orders: &[Order<'_>]) -> Vec<String> {
    let mut totals: BTreeMap<&str, (i64, usize)> = BTreeMap::new();

    for order in orders {
        let entry = totals.entry(order.month).or_insert((0, 0));
        entry.0 += order.amount_cents;
        entry.1 += 1;
    }

    totals
        .into_iter()
        .map(|(month, (amount, count))| format!("{}: {} orders ${}.{:02}", month, count, amount / 100, amount % 100))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_orders<'a>() -> Vec<Order<'a>> {
        vec![
            Order { month: "2024-01", region: "east", amount_cents: 1200, refunded: false },
            Order { month: "2024-01", region: "west", amount_cents: 500, refunded: true },
            Order { month: "2024-01", region: "east", amount_cents: 800, refunded: false },
            Order { month: "2024-02", region: "north", amount_cents: 900, refunded: false },
            Order { month: "2024-02", region: "east", amount_cents: 600, refunded: false },
            Order { month: "2024-02", region: "north", amount_cents: 400, refunded: true },
            Order { month: "2024-03", region: "south", amount_cents: 300, refunded: true },
        ]
    }

    #[test]
    fn groups_by_month_in_sorted_order_and_excludes_refunds() {
        let report = monthly_region_report(&sample_orders());
        assert_eq!(
            report,
            vec![
                "2024-01: 2 orders, 1 regions, total $20.00",
                "2024-02: 2 orders, 2 regions, total $15.00",
            ]
        );
    }

    #[test]
    fn omits_months_with_only_refunds() {
        let orders = vec![
            Order { month: "2024-04", region: "west", amount_cents: 700, refunded: true },
            Order { month: "2024-04", region: "east", amount_cents: 200, refunded: true },
        ];

        let report = monthly_region_report(&orders);
        assert!(report.is_empty());
    }
}
