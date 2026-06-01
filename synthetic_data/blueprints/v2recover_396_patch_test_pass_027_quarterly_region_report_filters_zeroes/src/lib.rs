use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub region: &'static str,
    pub amount_cents: i32,
    pub refunded: bool,
}

pub fn build_region_report(orders: &[Order]) -> String {
    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();
    for order in orders {
        *totals.entry(order.region).or_insert(0) += order.amount_cents;
    }

    let mut rows: Vec<(&str, i32)> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    let mut out = String::new();
    let grand_total: i32 = rows.iter().map(|(_, total)| *total).sum();
    for (region, total) in rows {
        out.push_str(&format!("{}:${:.2}\n", region, total as f64 / 100.0));
    }
    out.push_str(&format!("TOTAL:${:.2}", grand_total as f64 / 100.0));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_total_desc_then_region_and_ignores_refunds() {
        let orders = vec![
            Order { region: "west", amount_cents: 500, refunded: false },
            Order { region: "east", amount_cents: 200, refunded: false },
            Order { region: "north", amount_cents: 500, refunded: false },
            Order { region: "east", amount_cents: 700, refunded: true },
            Order { region: "south", amount_cents: 300, refunded: false },
        ];

        let report = build_region_report(&orders);
        assert_eq!(report, "north:$5.00\nwest:$5.00\nsouth:$3.00\neast:$2.00\nTOTAL:$15.00");
    }

    #[test]
    fn skips_regions_with_zero_non_refunded_total() {
        let orders = vec![
            Order { region: "alpha", amount_cents: 0, refunded: false },
            Order { region: "beta", amount_cents: 100, refunded: true },
            Order { region: "gamma", amount_cents: 250, refunded: false },
        ];

        let report = build_region_report(&orders);
        assert_eq!(report, "gamma:$2.50\nTOTAL:$2.50");
    }
}
