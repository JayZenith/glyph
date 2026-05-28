use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub region: &'static str,
    pub customer: &'static str,
    pub amount_cents: u32,
    pub completed: bool,
}

pub fn summarize(orders: &[Order]) -> String {
    let mut totals: BTreeMap<&str, (u32, u32)> = BTreeMap::new();

    for order in orders {
        let entry = totals.entry(order.region).or_insert((0, 0));
        entry.0 += order.amount_cents;
        entry.1 += 1;
    }

    let mut lines = Vec::new();
    for (region, (amount, customers)) in totals {
        lines.push(format!("{}: ${:.2} ({} customers)", region, amount as f64 / 100.0, customers));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarizes_only_completed_orders_and_unique_customers() {
        let orders = vec![
            Order { region: "West", customer: "Ava", amount_cents: 1200, completed: true },
            Order { region: "West", customer: "Ava", amount_cents: 800, completed: true },
            Order { region: "West", customer: "Ben", amount_cents: 500, completed: false },
            Order { region: "East", customer: "Cara", amount_cents: 700, completed: true },
            Order { region: "East", customer: "Cara", amount_cents: 300, completed: false },
        ];

        let report = summarize(&orders);
        assert_eq!(report, "East: $7.00 (1 customers)\nWest: $20.00 (1 customers)");
    }

    #[test]
    fn sorts_regions_and_skips_empty_result() {
        let orders = vec![
            Order { region: "South", customer: "Ivy", amount_cents: 900, completed: false },
            Order { region: "North", customer: "Noah", amount_cents: 250, completed: true },
            Order { region: "South", customer: "Lia", amount_cents: 1000, completed: true },
        ];

        let report = summarize(&orders);
        assert_eq!(report, "North: $2.50 (1 customers)\nSouth: $10.00 (1 customers)");
        assert_eq!(summarize(&[]), "");
    }

    #[test]
    fn unique_customer_count_does_not_double_count_repeat_buyers() {
        let orders = vec![
            Order { region: "Central", customer: "Mia", amount_cents: 100, completed: true },
            Order { region: "Central", customer: "Mia", amount_cents: 200, completed: true },
            Order { region: "Central", customer: "Omar", amount_cents: 300, completed: true },
        ];

        let report = summarize(&orders);
        assert_eq!(report, "Central: $6.00 (2 customers)");
    }
}
