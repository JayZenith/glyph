use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub category: String,
    pub amount: i32,
}

pub fn summarize(entries: &[Entry]) -> Vec<String> {
    let mut totals = BTreeMap::<String, i32>::new();
    for entry in entries {
        *totals.entry(entry.category.clone()).or_insert(0) += entry.amount;
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(&b.0));

    rows.into_iter()
        .map(|(category, total)| format!("{}:{}", category, total))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn e(category: &str, amount: i32) -> Entry {
        Entry { category: category.to_string(), amount }
    }

    #[test]
    fn groups_positive_amounts_and_orders_by_total_then_name() {
        let entries = vec![
            e("ops", 3),
            e("food", 7),
            e("ops", 5),
            e("misc", 2),
            e("food", -10),
            e("food", 4),
            e("misc", 6),
        ];

        assert_eq!(summarize(&entries), vec!["food:11", "misc:8", "ops:8"]);
    }

    #[test]
    fn skips_zero_or_negative_totals_after_ignoring_negative_entries() {
        let entries = vec![
            e("keep", 4),
            e("drop_zero", 0),
            e("drop_neg", -3),
            e("keep", 1),
            e("drop_zero", -8),
        ];

        assert_eq!(summarize(&entries), vec!["keep:5"]);
    }
}
