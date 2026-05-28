#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub region: &'static str,
    pub units: u32,
    pub unit_price_cents: i32,
    pub returned: bool,
}

pub fn summarize_by_region(orders: &[Order]) -> Vec<String> {
    let mut rows: Vec<(&str, u32, i32)> = Vec::new();

    for order in orders {
        let mut found = false;
        for row in rows.iter_mut() {
            if row.0 == order.region {
                row.1 += order.units;
                row.2 += order.units as i32 * order.unit_price_cents;
                found = true;
                break;
            }
        }
        if !found {
            rows.push((
                order.region,
                order.units,
                order.units as i32 * order.unit_price_cents,
            ));
        }
    }

    rows.sort_by(|a, b| a.0.cmp(b.0));

    rows.into_iter()
        .map(|(region, units, revenue)| format!("{}: {} units ${}.{:02}", region, units, revenue / 100, revenue % 100))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_returns_and_zero_priced_revenue_then_sorts_by_revenue_desc() {
        let orders = vec![
            Order { region: "west", units: 3, unit_price_cents: 250, returned: false },
            Order { region: "east", units: 1, unit_price_cents: 500, returned: true },
            Order { region: "north", units: 2, unit_price_cents: 0, returned: false },
            Order { region: "east", units: 4, unit_price_cents: 200, returned: false },
            Order { region: "west", units: 1, unit_price_cents: 250, returned: false },
        ];

        assert_eq!(
            summarize_by_region(&orders),
            vec![
                "west | units=4 | revenue=$10.00",
                "east | units=4 | revenue=$8.00",
                "north | units=2 | revenue=$0.00",
            ]
        );
    }

    #[test]
    fn ties_on_revenue_break_by_region_name() {
        let orders = vec![
            Order { region: "beta", units: 1, unit_price_cents: 300, returned: false },
            Order { region: "alpha", units: 3, unit_price_cents: 100, returned: false },
            Order { region: "gamma", units: 2, unit_price_cents: 50, returned: true },
        ];

        assert_eq!(
            summarize_by_region(&orders),
            vec![
                "alpha | units=3 | revenue=$3.00",
                "beta | units=1 | revenue=$3.00",
            ]
        );
    }
}
