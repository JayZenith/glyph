use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Entry {
    pub category: &'static str,
    pub amount: i32,
}

pub fn build_report(entries: &[Entry]) -> String {
    let mut totals: BTreeMap<&str, (i32, usize)> = BTreeMap::new();

    for entry in entries {
        let item = totals.entry(entry.category).or_insert((0, 0));
        item.0 += entry.amount;
        item.1 += 1;
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    rows.into_iter()
        .map(|(category, (total, count))| format!("{}: total={} count={}", category, total, count))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::{build_report, Entry};

    #[test]
    fn ignores_non_positive_amounts_and_sorts_by_total_desc_then_name() {
        let entries = [
            Entry { category: "books", amount: 30 },
            Entry { category: "games", amount: 30 },
            Entry { category: "books", amount: 10 },
            Entry { category: "food", amount: 0 },
            Entry { category: "games", amount: -5 },
            Entry { category: "food", amount: 20 },
            Entry { category: "books", amount: -2 },
        ];

        let report = build_report(&entries);
        assert_eq!(report, "books | total=40 | items=2\ngames | total=30 | items=1\nfood | total=20 | items=1");
    }

    #[test]
    fn returns_empty_string_when_nothing_positive_exists() {
        let entries = [
            Entry { category: "books", amount: 0 },
            Entry { category: "games", amount: -3 },
        ];

        assert_eq!(build_report(&entries), "");
    }
}
