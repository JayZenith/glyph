use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Completed,
    Pending,
    Refunded,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub category: &'static str,
    pub amount: f64,
    pub status: Status,
}

pub fn sales_report(orders: &[Order]) -> String {
    let mut totals: BTreeMap<&str, f64> = BTreeMap::new();

    for order in orders {
        *totals.entry(order.category).or_insert(0.0) += order.amount;
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    rows.into_iter()
        .map(|(category, total)| format!("{}:{:.2}", category, total))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_filters_completed_positive_and_sorts_by_total_then_name() {
        let orders = vec![
            Order { category: "hardware", amount: 12.0, status: Status::Completed },
            Order { category: "software", amount: 9.5, status: Status::Completed },
            Order { category: "books", amount: 12.0, status: Status::Completed },
            Order { category: "books", amount: -1.0, status: Status::Completed },
            Order { category: "garden", amount: 0.0, status: Status::Completed },
            Order { category: "software", amount: 2.0, status: Status::Pending },
            Order { category: "hardware", amount: 5.0, status: Status::Refunded },
        ];

        assert_eq!(sales_report(&orders), "books:12\nhardware:12\nsoftware:9.5");
    }

    #[test]
    fn report_uses_trimmed_decimals_and_empty_when_nothing_counts() {
        let orders = vec![
            Order { category: "alpha", amount: 2.25, status: Status::Completed },
            Order { category: "alpha", amount: 0.75, status: Status::Completed },
            Order { category: "beta", amount: 1.2, status: Status::Completed },
            Order { category: "beta", amount: -1.2, status: Status::Completed },
            Order { category: "gamma", amount: 4.0, status: Status::Pending },
        ];

        assert_eq!(sales_report(&orders), "alpha:3\nbeta:1.2");
        assert_eq!(sales_report(&[Order { category: "x", amount: 0.0, status: Status::Completed }]), "");
    }
}
