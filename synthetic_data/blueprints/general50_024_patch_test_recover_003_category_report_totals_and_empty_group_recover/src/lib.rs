use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Entry {
    pub category: &'static str,
    pub amount: i32,
    pub approved: bool,
}

pub fn build_report(entries: &[Entry]) -> String {
    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();

    for entry in entries {
        if entry.approved {
            *totals.entry(entry.category).or_insert(0) += entry.amount;
        }
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    let mut out = String::new();
    for (category, total) in rows {
        out.push_str(&format!("{}:{}\n", category, total));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{build_report, Entry};

    #[test]
    fn approved_positive_totals_sorted_by_total_then_name() {
        let entries = [
            Entry { category: "ops", amount: 5, approved: true },
            Entry { category: "sales", amount: 4, approved: true },
            Entry { category: "ops", amount: -2, approved: true },
            Entry { category: "hr", amount: 4, approved: true },
            Entry { category: "sales", amount: 3, approved: false },
            Entry { category: "legal", amount: 0, approved: true },
        ];

        assert_eq!(build_report(&entries), "ops:5\nhr:4\nsales:4");
    }

    #[test]
    fn empty_after_filter_returns_no_data() {
        let entries = [
            Entry { category: "ops", amount: -2, approved: true },
            Entry { category: "sales", amount: 7, approved: false },
            Entry { category: "hr", amount: 0, approved: true },
        ];

        assert_eq!(build_report(&entries), "no data");
    }
}
