use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order<'a> {
    pub region: &'a str,
    pub week: u32,
    pub units: u32,
    pub returned: bool,
}

pub fn summarize(orders: &[Order<'_>]) -> String {
    let mut totals: BTreeMap<(u32, &str), u32> = BTreeMap::new();

    for order in orders {
        if order.returned {
            continue;
        }
        *totals.entry((order.week, order.region)).or_insert(0) += order.units;
    }

    let mut lines = Vec::new();
    for ((week, region), units) in totals {
        lines.push(format!("W{} {} total={}", week, region, units));
    }

    if lines.is_empty() {
        "no shipments".to_string()
    } else {
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_by_week_then_region_and_ignores_returns() {
        let orders = [
            Order { region: "west", week: 2, units: 5, returned: false },
            Order { region: "east", week: 1, units: 4, returned: false },
            Order { region: "west", week: 2, units: 3, returned: true },
            Order { region: "east", week: 1, units: 6, returned: false },
            Order { region: "west", week: 1, units: 2, returned: false },
        ];

        let got = summarize(&orders);
        let expected = "W1 east total=10 orders=2\nW1 west total=2 orders=1\nW2 west total=5 orders=1";
        assert_eq!(got, expected);
    }

    #[test]
    fn skips_zero_unit_shipments_but_reports_empty_when_nothing_counts() {
        let orders = [
            Order { region: "north", week: 3, units: 0, returned: false },
            Order { region: "north", week: 3, units: 2, returned: true },
        ];

        assert_eq!(summarize(&orders), "no shipments");
    }

    #[test]
    fn excludes_groups_with_zero_total_even_if_non_returned_orders_exist() {
        let orders = [
            Order { region: "south", week: 4, units: 0, returned: false },
            Order { region: "south", week: 4, units: 0, returned: false },
            Order { region: "east", week: 4, units: 1, returned: false },
        ];

        let got = summarize(&orders);
        assert_eq!(got, "W4 east total=1 orders=1");
    }
}
