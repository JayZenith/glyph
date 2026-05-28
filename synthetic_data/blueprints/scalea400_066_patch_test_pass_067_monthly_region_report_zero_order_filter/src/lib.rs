use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Order {
    pub region: &'static str,
    pub month: &'static str,
    pub amount: i32,
    pub shipped: bool,
}

pub fn monthly_region_report(orders: &[Order]) -> String {
    let mut totals: BTreeMap<&str, BTreeMap<&str, i32>> = BTreeMap::new();

    for order in orders {
        if !order.shipped {
            continue;
        }
        *totals
            .entry(order.region)
            .or_default()
            .entry(order.month)
            .or_default() += order.amount;
    }

    let mut out = String::new();
    for (region, months) in totals {
        out.push_str(region);
        out.push(':');
        let mut first = true;
        for (month, total) in months {
            if !first {
                out.push(',');
            }
            first = false;
            out.push(' ');
            out.push_str(month);
            out.push('=');
            out.push_str(&total.to_string());
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_shipped_totals_by_region_and_month() {
        let orders = vec![
            Order { region: "West", month: "2024-02", amount: 5, shipped: true },
            Order { region: "East", month: "2024-01", amount: 7, shipped: true },
            Order { region: "West", month: "2024-01", amount: 3, shipped: true },
            Order { region: "East", month: "2024-01", amount: 2, shipped: true },
            Order { region: "West", month: "2024-02", amount: 4, shipped: false },
        ];

        assert_eq!(
            monthly_region_report(&orders),
            "East: 2024-01=9\nWest: 2024-01=3, 2024-02=5\n"
        );
    }

    #[test]
    fn skips_zero_and_negative_shipped_amounts() {
        let orders = vec![
            Order { region: "North", month: "2024-03", amount: 10, shipped: true },
            Order { region: "North", month: "2024-03", amount: 0, shipped: true },
            Order { region: "North", month: "2024-03", amount: -4, shipped: true },
            Order { region: "North", month: "2024-04", amount: 8, shipped: true },
            Order { region: "North", month: "2024-04", amount: -2, shipped: false },
        ];

        assert_eq!(
            monthly_region_report(&orders),
            "North: 2024-03=10, 2024-04=8\n"
        );
    }
}
