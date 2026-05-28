#[derive(Clone, Debug)]
pub struct Order {
    pub region: &'static str,
    pub amount: i32,
    pub cancelled: bool,
}

pub fn sales_report(orders: &[Order]) -> Vec<String> {
    let mut totals: Vec<(&str, i32)> = Vec::new();

    for order in orders {
        if order.cancelled {
            continue;
        }

        if let Some((_, total)) = totals.iter_mut().find(|(region, _)| *region == order.region) {
            *total += order.amount;
        } else {
            totals.push((order.region, order.amount));
        }
    }

    totals.sort_by(|a, b| a.0.cmp(b.0));

    totals
        .into_iter()
        .map(|(region, total)| format!("{}:{}", region, total))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregates_filters_and_orders_report() {
        let orders = vec![
            Order { region: "west", amount: 20, cancelled: false },
            Order { region: "east", amount: 40, cancelled: false },
            Order { region: "west", amount: -5, cancelled: false },
            Order { region: "north", amount: 10, cancelled: true },
            Order { region: "south", amount: 15, cancelled: false },
            Order { region: "east", amount: -10, cancelled: false },
            Order { region: "south", amount: -15, cancelled: false },
            Order { region: "north", amount: 30, cancelled: false },
        ];

        assert_eq!(
            sales_report(&orders),
            vec![
                "east:30".to_string(),
                "north:30".to_string(),
                "west:15".to_string(),
            ]
        );
    }

    #[test]
    fn tiebreaks_same_total_by_region_name() {
        let orders = vec![
            Order { region: "delta", amount: 12, cancelled: false },
            Order { region: "alpha", amount: 12, cancelled: false },
            Order { region: "zeta", amount: 5, cancelled: true },
        ];

        assert_eq!(
            sales_report(&orders),
            vec!["alpha:12".to_string(), "delta:12".to_string()]
        );
    }
}
