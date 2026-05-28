use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub category: &'static str,
    pub amount: i64,
    pub refunded: bool,
}

pub fn category_report(entries: &[Entry]) -> Vec<String> {
    let mut totals: BTreeMap<&'static str, i64> = BTreeMap::new();

    for entry in entries {
        *totals.entry(entry.category).or_insert(0) += entry.amount;
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    rows.into_iter()
        .map(|(category, total)| format!("{}:{}", category, total))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{category_report, Entry};

    #[test]
    fn ignores_refunds_and_non_positive_totals_then_sorts() {
        let entries = vec![
            Entry { category: "books", amount: 20, refunded: false },
            Entry { category: "games", amount: 35, refunded: false },
            Entry { category: "books", amount: 15, refunded: true },
            Entry { category: "music", amount: 35, refunded: false },
            Entry { category: "garden", amount: -5, refunded: false },
            Entry { category: "games", amount: 5, refunded: true },
            Entry { category: "toys", amount: 0, refunded: false },
        ];

        assert_eq!(
            category_report(&entries),
            vec![
                "games:35".to_string(),
                "music:35".to_string(),
                "books:20".to_string(),
            ]
        );
    }

    #[test]
    fn combines_duplicate_categories_and_drops_zeroed_results() {
        let entries = vec![
            Entry { category: "office", amount: 8, refunded: false },
            Entry { category: "office", amount: 2, refunded: false },
            Entry { category: "kitchen", amount: 10, refunded: false },
            Entry { category: "kitchen", amount: -10, refunded: false },
            Entry { category: "office", amount: 4, refunded: true },
        ];

        assert_eq!(category_report(&entries), vec!["office:10".to_string()]);
    }
}
