use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale<'a> {
    pub region: &'a str,
    pub rep: &'a str,
    pub amount: i32,
    pub refunded: bool,
}

pub fn summarize_sales(sales: &[Sale<'_>]) -> Vec<String> {
    let mut totals: BTreeMap<&str, (i32, i32)> = BTreeMap::new();

    for sale in sales {
        if sale.refunded {
            continue;
        }

        let entry = totals.entry(sale.region).or_insert((0, 0));
        entry.0 += sale.amount;
        entry.1 += 1;
    }

    let mut rows: Vec<_> = totals
        .into_iter()
        .map(|(region, (total, count))| format!("{}: total={}, reps={}", region, total, count))
        .collect();

    rows.sort();
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_totals_and_unique_rep_counts() {
        let sales = [
            Sale { region: "west", rep: "amy", amount: 40, refunded: false },
            Sale { region: "east", rep: "bob", amount: 25, refunded: false },
            Sale { region: "west", rep: "amy", amount: 10, refunded: false },
            Sale { region: "west", rep: "cara", amount: 15, refunded: false },
            Sale { region: "east", rep: "dina", amount: 35, refunded: false },
        ];

        assert_eq!(
            summarize_sales(&sales),
            vec![
                "west: total=65, reps=2".to_string(),
                "east: total=60, reps=2".to_string(),
            ]
        );
    }

    #[test]
    fn ignores_refunded_and_non_positive_sales() {
        let sales = [
            Sale { region: "north", rep: "ivy", amount: 20, refunded: false },
            Sale { region: "north", rep: "ivy", amount: 0, refunded: false },
            Sale { region: "south", rep: "jo", amount: 50, refunded: true },
            Sale { region: "south", rep: "kai", amount: -5, refunded: false },
        ];

        assert_eq!(
            summarize_sales(&sales),
            vec!["north: total=20, reps=1".to_string()]
        );
    }

    #[test]
    fn sorts_by_total_desc_then_region_asc() {
        let sales = [
            Sale { region: "beta", rep: "r1", amount: 30, refunded: false },
            Sale { region: "alpha", rep: "r2", amount: 30, refunded: false },
            Sale { region: "gamma", rep: "r3", amount: 45, refunded: false },
        ];

        assert_eq!(
            summarize_sales(&sales),
            vec![
                "gamma: total=45, reps=1".to_string(),
                "alpha: total=30, reps=1".to_string(),
                "beta: total=30, reps=1".to_string(),
            ]
        );
    }
}
