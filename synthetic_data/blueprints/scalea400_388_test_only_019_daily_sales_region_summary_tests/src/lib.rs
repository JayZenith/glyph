use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale {
    pub region: &'static str,
    pub amount: u32,
    pub refunded: bool,
}

pub fn summarize_by_region(sales: &[Sale]) -> Vec<String> {
    let mut totals: BTreeMap<&str, (u32, u32)> = BTreeMap::new();

    for sale in sales {
        if sale.refunded {
            continue;
        }
        let entry = totals.entry(sale.region).or_insert((0, 0));
        entry.0 += sale.amount;
        entry.1 += 1;
    }

    totals
        .into_iter()
        .map(|(region, (amount, count))| format!("{region}: {count} sale(s), total={amount}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_totals_and_counts_per_region() {
        let sales = vec![
            Sale { region: "east", amount: 40, refunded: false },
            Sale { region: "west", amount: 15, refunded: false },
            Sale { region: "east", amount: 10, refunded: false },
        ];

        assert_eq!(
            summarize_by_region(&sales),
            vec![
                "east: 2 sale(s), total=50",
                "west: 1 sale(s), total=15",
            ]
        );
    }

    #[test]
    fn excludes_refunded_sales() {
        let sales = vec![
            Sale { region: "north", amount: 25, refunded: true },
            Sale { region: "north", amount: 30, refunded: false },
            Sale { region: "south", amount: 5, refunded: true },
        ];

        assert_eq!(
            summarize_by_region(&sales),
            vec!["north: 1 sale(s), total=30"]
        );
    }

    #[test]
    fn returns_sorted_regions_and_empty_for_no_kept_sales() {
        let sales = vec![
            Sale { region: "delta", amount: 8, refunded: false },
            Sale { region: "alpha", amount: 3, refunded: false },
            Sale { region: "charlie", amount: 9, refunded: false },
        ];

        assert_eq!(
            summarize_by_region(&sales),
            vec![
                "alpha: 1 sale(s), total=3",
                "charlie: 1 sale(s), total=9",
                "delta: 1 sale(s), total=8",
            ]
        );

        let refunded_only = vec![
            Sale { region: "x", amount: 1, refunded: true },
            Sale { region: "y", amount: 2, refunded: true },
        ];
        assert!(summarize_by_region(&refunded_only).is_empty());
    }
}
