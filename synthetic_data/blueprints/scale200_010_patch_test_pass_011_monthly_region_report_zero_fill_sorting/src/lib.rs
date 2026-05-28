use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Order {
    pub region: &'static str,
    pub month: u8,
    pub amount: u32,
    pub shipped: bool,
}

pub fn build_report(orders: &[Order]) -> String {
    let mut grouped: BTreeMap<&str, BTreeMap<u8, u32>> = BTreeMap::new();

    for order in orders {
        let region_entry = grouped.entry(order.region).or_default();
        *region_entry.entry(order.month).or_default() += order.amount;
    }

    let mut rows: Vec<(&str, BTreeMap<u8, u32>, u32)> = grouped
        .into_iter()
        .map(|(region, months)| {
            let total = months.values().copied().sum();
            (region, months, total)
        })
        .collect();

    rows.sort_by(|a, b| a.0.cmp(b.0));

    let mut out = String::new();
    for (idx, (region, months, total)) in rows.iter().enumerate() {
        if idx > 0 {
            out.push('\n');
        }
        out.push_str(region);
        out.push_str(" total=");
        out.push_str(&total.to_string());
        out.push(':');
        for (month, amount) in months {
            out.push(' ');
            out.push_str(&month.to_string());
            out.push('=');
            out.push_str(&amount.to_string());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{build_report, Order};

    #[test]
    fn groups_shipped_orders_and_sorts_by_total_then_name() {
        let orders = vec![
            Order { region: "west", month: 2, amount: 50, shipped: true },
            Order { region: "west", month: 4, amount: 25, shipped: true },
            Order { region: "west", month: 3, amount: 999, shipped: false },
            Order { region: "east", month: 1, amount: 30, shipped: true },
            Order { region: "east", month: 3, amount: 20, shipped: true },
            Order { region: "east", month: 2, amount: 7, shipped: false },
            Order { region: "north", month: 5, amount: 75, shipped: true },
            Order { region: "south", month: 1, amount: 10, shipped: false },
        ];

        let report = build_report(&orders);
        assert_eq!(
            report,
            "north total=75: 5=75\nwest total=75: 2=50 3=0 4=25\neast total=50: 1=30 2=0 3=20"
        );
    }

    #[test]
    fn region_with_only_unshipped_orders_is_excluded() {
        let orders = vec![
            Order { region: "alpha", month: 6, amount: 10, shipped: false },
            Order { region: "beta", month: 6, amount: 5, shipped: true },
        ];

        assert_eq!(build_report(&orders), "beta total=5: 6=5");
    }
}
