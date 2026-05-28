use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale {
    pub region: &'static str,
    pub month: u8,
    pub units: u32,
    pub revenue_cents: u32,
}

pub fn monthly_region_summary(sales: &[Sale]) -> String {
    let mut totals: BTreeMap<(u8, &'static str), (u32, u32)> = BTreeMap::new();

    for sale in sales {
        let entry = totals.entry((sale.month, sale.region)).or_insert((0, 0));
        entry.0 += sale.units;
        entry.1 += sale.revenue_cents;
    }

    let mut out = Vec::new();
    for ((month, region), (units, revenue_cents)) in totals {
        out.push(format!(
            "M{:02} {}: {} units ${}.{:02}",
            month,
            region,
            units,
            revenue_cents / 100,
            revenue_cents % 100
        ));
    }

    out.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregates_and_sorts_by_month_then_region() {
        let sales = [
            Sale { region: "West", month: 2, units: 3, revenue_cents: 1200 },
            Sale { region: "East", month: 1, units: 2, revenue_cents: 500 },
            Sale { region: "East", month: 1, units: 1, revenue_cents: 250 },
            Sale { region: "West", month: 1, units: 4, revenue_cents: 0 },
            Sale { region: "East", month: 2, units: 1, revenue_cents: 300 },
        ];

        let got = monthly_region_summary(&sales);
        let want = "M01 East: 3 units $7.50\nM02 East: 1 units $3.00\nM02 West: 3 units $12.00";
        assert_eq!(got, want);
    }

    #[test]
    fn drops_groups_with_zero_total_revenue_even_if_units_exist() {
        let sales = [
            Sale { region: "North", month: 4, units: 2, revenue_cents: 0 },
            Sale { region: "North", month: 4, units: 1, revenue_cents: 0 },
            Sale { region: "South", month: 4, units: 5, revenue_cents: 999 },
        ];

        let got = monthly_region_summary(&sales);
        assert_eq!(got, "M04 South: 5 units $9.99");
    }

    #[test]
    fn returns_empty_string_when_all_groups_are_zero_revenue() {
        let sales = [
            Sale { region: "North", month: 7, units: 10, revenue_cents: 0 },
            Sale { region: "South", month: 7, units: 1, revenue_cents: 0 },
        ];

        assert_eq!(monthly_region_summary(&sales), "");
    }
}
