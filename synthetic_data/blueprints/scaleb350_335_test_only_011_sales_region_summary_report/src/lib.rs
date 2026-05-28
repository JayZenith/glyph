use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale {
    pub region: &'static str,
    pub amount: u32,
    pub refunded: bool,
}

pub fn summarize_sales(sales: &[Sale]) -> Vec<String> {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();

    for sale in sales {
        if !sale.refunded {
            *totals.entry(sale.region).or_insert(0) += sale.amount;
        }
    }

    totals
        .into_iter()
        .map(|(region, total)| format!("{}: {}", region, total))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{summarize_sales, Sale};

    #[test]
    fn groups_non_refunded_sales_by_region() {
        let sales = [
            Sale { region: "east", amount: 15, refunded: false },
            Sale { region: "west", amount: 7, refunded: false },
            Sale { region: "east", amount: 5, refunded: false },
        ];

        let report = summarize_sales(&sales);
        assert_eq!(report, vec!["east: 20", "west: 7"]);
    }

    #[test]
    fn skips_refunded_sales_and_omits_empty_regions() {
        let sales = [
            Sale { region: "north", amount: 9, refunded: true },
            Sale { region: "south", amount: 4, refunded: false },
            Sale { region: "south", amount: 6, refunded: true },
        ];

        let report = summarize_sales(&sales);
        assert_eq!(report, vec!["south: 4"]);
    }

    #[test]
    fn empty_input_produces_empty_report() {
        let report = summarize_sales(&[]);
        assert!(report.is_empty());
    }
}
