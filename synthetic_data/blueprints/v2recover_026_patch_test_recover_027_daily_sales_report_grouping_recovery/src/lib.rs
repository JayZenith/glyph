use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
pub struct Order {
    pub region: &'static str,
    pub amount_cents: u32,
    pub refunded: bool,
}

pub fn sales_report(orders: &[Order]) -> Vec<String> {
    let mut totals: BTreeMap<&str, (u32, u32)> = BTreeMap::new();

    for order in orders {
        let entry = totals.entry(order.region).or_insert((0, 0));
        entry.0 += order.amount_cents;
        entry.1 += 1;
    }

    let mut rows: Vec<_> = totals
        .into_iter()
        .map(|(region, (total, count))| (region, total, count))
        .collect();

    rows.sort_by(|a, b| a.0.cmp(b.0));

    rows
        .into_iter()
        .map(|(region, total, count)| format!("{}: {} orders, ${}.{:02}", region, count, total / 100, total % 100))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_non_refunded_sales_and_sorts_by_total_desc_then_region() {
        let orders = [
            Order { region: "West", amount_cents: 1200, refunded: false },
            Order { region: "East", amount_cents: 500, refunded: true },
            Order { region: "East", amount_cents: 2000, refunded: false },
            Order { region: "West", amount_cents: 300, refunded: false },
            Order { region: "North", amount_cents: 1500, refunded: false },
            Order { region: "East", amount_cents: 700, refunded: false },
        ];

        let report = sales_report(&orders);
        assert_eq!(
            report,
            vec![
                "East | orders=2 | gross=$27.00",
                "North | orders=1 | gross=$15.00",
                "West | orders=2 | gross=$15.00",
            ]
        );
    }

    #[test]
    fn returns_empty_when_everything_is_refunded() {
        let orders = [
            Order { region: "South", amount_cents: 999, refunded: true },
            Order { region: "East", amount_cents: 1200, refunded: true },
        ];

        assert!(sales_report(&orders).is_empty());
    }
}
