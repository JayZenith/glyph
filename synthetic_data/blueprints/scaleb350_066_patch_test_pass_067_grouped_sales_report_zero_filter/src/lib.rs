use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale<'a> {
    pub category: &'a str,
    pub qty: i32,
}

pub fn category_report(sales: &[Sale<'_>]) -> Vec<String> {
    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();
    for sale in sales {
        *totals.entry(sale.category).or_insert(0) += sale.qty;
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    rows.into_iter()
        .map(|(category, total)| format!("{}:{}", category, total))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_total_desc_then_category_and_skips_zero_totals() {
        let sales = [
            Sale { category: "hardware", qty: 3 },
            Sale { category: "books", qty: 5 },
            Sale { category: "games", qty: 1 },
            Sale { category: "hardware", qty: -1 },
            Sale { category: "games", qty: -1 },
            Sale { category: "garden", qty: 2 },
            Sale { category: "garden", qty: 3 },
        ];

        assert_eq!(
            category_report(&sales),
            vec!["books:5", "garden:5", "hardware:2"]
        );
    }

    #[test]
    fn handles_empty_input() {
        let sales: [Sale<'_>; 0] = [];
        let report: Vec<String> = Vec::new();
        assert_eq!(category_report(&sales), report);
    }
}
