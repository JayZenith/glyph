use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub category: &'static str,
    pub amount: i32,
    pub active: bool,
}

pub fn summarize(entries: &[Entry]) -> String {
    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();

    for entry in entries {
        if entry.active {
            *totals.entry(entry.category).or_insert(0) += entry.amount;
        }
    }

    let mut lines = Vec::new();
    let grand_total: i32 = totals.values().copied().sum();
    lines.push(format!("categories={}", totals.len()));

    for (category, total) in totals {
        lines.push(format!("{}={}", category, total));
    }

    lines.push(format!("grand_total={}", grand_total));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_sorted_active_category_totals() {
        let entries = vec![
            Entry { category: "tools", amount: 4, active: true },
            Entry { category: "food", amount: 7, active: false },
            Entry { category: "books", amount: 3, active: true },
            Entry { category: "tools", amount: 6, active: true },
            Entry { category: "books", amount: 2, active: true },
        ];

        let report = summarize(&entries);
        assert_eq!(report, "categories=2\nbooks=5\ntools=10\ngrand_total=15");
    }

    #[test]
    fn empty_when_no_active_entries() {
        let entries = vec![
            Entry { category: "food", amount: 7, active: false },
            Entry { category: "tools", amount: 6, active: false },
        ];

        let report = summarize(&entries);
        assert_eq!(report, "categories=0\ngrand_total=0");
    }
}
