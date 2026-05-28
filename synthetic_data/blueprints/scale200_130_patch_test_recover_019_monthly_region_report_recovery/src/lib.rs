use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Sale {
    pub month: &'static str,
    pub region: &'static str,
    pub amount: i32,
    pub refunded: bool,
}

pub fn monthly_region_report(sales: &[Sale]) -> String {
    let mut totals: BTreeMap<&str, (i32, usize)> = BTreeMap::new();

    for sale in sales {
        let entry = totals.entry(sale.month).or_insert((0, 0));
        entry.0 += sale.amount;
        entry.1 += 1;
    }

    let mut out = String::new();
    for (month, (total, count)) in totals {
        out.push_str(&format!("{}: total={} count={}\n", month, total, count));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_sales() -> Vec<Sale> {
        vec![
            Sale { month: "2024-01", region: "north", amount: 10, refunded: false },
            Sale { month: "2024-01", region: "north", amount: 5, refunded: true },
            Sale { month: "2024-01", region: "south", amount: 8, refunded: false },
            Sale { month: "2024-02", region: "north", amount: 12, refunded: false },
            Sale { month: "2024-02", region: "south", amount: 4, refunded: true },
            Sale { month: "2024-02", region: "south", amount: 7, refunded: false },
        ]
    }

    #[test]
    fn groups_by_month_and_region_and_skips_refunds() {
        let report = monthly_region_report(&sample_sales());
        let expected = concat!(
            "2024-01\n",
            "  north: total=10 count=1\n",
            "  south: total=8 count=1\n",
            "2024-02\n",
            "  north: total=12 count=1\n",
            "  south: total=7 count=1\n"
        );
        assert_eq!(report, expected);
    }

    #[test]
    fn sorts_regions_within_each_month() {
        let sales = vec![
            Sale { month: "2024-03", region: "west", amount: 2, refunded: false },
            Sale { month: "2024-03", region: "east", amount: 3, refunded: false },
            Sale { month: "2024-03", region: "central", amount: 4, refunded: false },
        ];
        let report = monthly_region_report(&sales);
        let expected = concat!(
            "2024-03\n",
            "  central: total=4 count=1\n",
            "  east: total=3 count=1\n",
            "  west: total=2 count=1\n"
        );
        assert_eq!(report, expected);
    }
}
