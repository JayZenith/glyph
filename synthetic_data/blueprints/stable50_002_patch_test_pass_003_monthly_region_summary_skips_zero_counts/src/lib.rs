use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale {
    pub region: &'static str,
    pub month: u8,
    pub amount: i32,
}

pub fn summarize_sales(sales: &[Sale]) -> Vec<String> {
    let mut grouped: BTreeMap<&'static str, (i32, usize, u8)> = BTreeMap::new();

    for sale in sales {
        let entry = grouped.entry(sale.region).or_insert((0, 0, sale.month));
        if sale.month < entry.2 {
            entry.2 = sale.month;
        }
        entry.0 += sale.amount;
        entry.1 += 1;
    }

    let mut lines = Vec::new();
    for (region, (total, count, first_month)) in grouped {
        if total <= 0 {
            continue;
        }
        lines.push(format!("{region}: total={total}, avg={}, first_month={first_month}", total / count as i32));
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::{summarize_sales, Sale};

    #[test]
    fn groups_by_region_and_sorts_alphabetically() {
        let sales = [
            Sale { region: "South", month: 5, amount: 12 },
            Sale { region: "North", month: 3, amount: 10 },
            Sale { region: "North", month: 1, amount: 20 },
        ];

        assert_eq!(
            summarize_sales(&sales),
            vec![
                "North: total=30, avg=15, first_month=1",
                "South: total=12, avg=12, first_month=5",
            ]
        );
    }

    #[test]
    fn excludes_regions_with_non_positive_totals() {
        let sales = [
            Sale { region: "East", month: 2, amount: 5 },
            Sale { region: "East", month: 4, amount: -5 },
            Sale { region: "West", month: 1, amount: -3 },
            Sale { region: "West", month: 2, amount: 1 },
        ];

        let summary = summarize_sales(&sales);
        assert!(summary.is_empty());
    }

    #[test]
    fn average_uses_only_positive_entries_for_included_regions() {
        let sales = [
            Sale { region: "Central", month: 6, amount: 10 },
            Sale { region: "Central", month: 4, amount: 0 },
            Sale { region: "Central", month: 2, amount: 20 },
        ];

        assert_eq!(
            summarize_sales(&sales),
            vec!["Central: total=30, avg=15, first_month=2"]
        );
    }
}
