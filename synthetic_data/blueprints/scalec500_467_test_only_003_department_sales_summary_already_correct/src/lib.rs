use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale {
    pub department: &'static str,
    pub amount: u32,
}

pub fn summarize_sales(sales: &[Sale], min_total: u32) -> Vec<String> {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();

    for sale in sales {
        *totals.entry(sale.department).or_insert(0) += sale.amount;
    }

    totals
        .into_iter()
        .filter(|(_, total)| *total >= min_total)
        .map(|(dept, total)| format!("{}:{}", dept, total))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_totals_and_filters_small_departments() {
        let sales = vec![
            Sale { department: "books", amount: 15 },
            Sale { department: "games", amount: 40 },
            Sale { department: "books", amount: 10 },
            Sale { department: "garden", amount: 8 },
            Sale { department: "games", amount: 5 },
        ];

        let summary = summarize_sales(&sales, 20);
        assert_eq!(summary, vec!["books:25", "games:45"]);
    }

    #[test]
    fn keeps_departments_exactly_at_threshold() {
        let sales = vec![
            Sale { department: "kitchen", amount: 12 },
            Sale { department: "kitchen", amount: 8 },
            Sale { department: "office", amount: 19 },
        ];

        let summary = summarize_sales(&sales, 20);
        assert_eq!(summary, vec!["kitchen:20"]);
    }

    #[test]
    fn returns_sorted_report_for_empty_or_multiple_entries() {
        let empty: Vec<Sale> = vec![];
        assert!(summarize_sales(&empty, 1).is_empty());

        let sales = vec![
            Sale { department: "zoo", amount: 9 },
            Sale { department: "auto", amount: 9 },
            Sale { department: "zoo", amount: 2 },
            Sale { department: "auto", amount: 3 },
        ];

        let summary = summarize_sales(&sales, 10);
        assert_eq!(summary, vec!["auto:12", "zoo:11"]);
    }
}
