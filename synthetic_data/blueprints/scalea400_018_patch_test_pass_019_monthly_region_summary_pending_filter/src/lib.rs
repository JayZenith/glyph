#[derive(Clone, Debug)]
pub struct Order<'a> {
    pub region: &'a str,
    pub amount: i32,
    pub shipped: bool,
}

pub fn summarize(orders: &[Order<'_>]) -> Vec<String> {
    let mut rows: Vec<(&str, usize, i32)> = Vec::new();

    for order in orders {
        let mut found = false;
        for row in &mut rows {
            if row.0 == order.region {
                row.1 += 1;
                row.2 += order.amount;
                found = true;
                break;
            }
        }
        if !found {
            rows.push((order.region, 1, order.amount));
        }
    }

    rows.sort_by(|a, b| a.0.cmp(b.0));

    rows.into_iter()
        .map(|(region, count, revenue)| format!("{region}: {count} orders (${revenue})"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{summarize, Order};

    #[test]
    fn groups_only_shipped_orders_and_ignores_non_positive_amounts_in_revenue() {
        let orders = vec![
            Order { region: "west", amount: 50, shipped: true },
            Order { region: "east", amount: 40, shipped: false },
            Order { region: "west", amount: -10, shipped: true },
            Order { region: "east", amount: 10, shipped: true },
            Order { region: "north", amount: 0, shipped: true },
        ];

        let got = summarize(&orders);
        assert_eq!(got, vec![
            "west: 2 orders ($50)",
            "east: 1 orders ($10)",
            "north: 1 orders ($0)",
        ]);
    }

    #[test]
    fn sorts_by_revenue_desc_then_region_name() {
        let orders = vec![
            Order { region: "beta", amount: 30, shipped: true },
            Order { region: "alpha", amount: 30, shipped: true },
            Order { region: "gamma", amount: 20, shipped: true },
            Order { region: "zeta", amount: 100, shipped: false },
        ];

        let got = summarize(&orders);
        assert_eq!(got, vec![
            "alpha: 1 orders ($30)",
            "beta: 1 orders ($30)",
            "gamma: 1 orders ($20)",
        ]);
    }
}
