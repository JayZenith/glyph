use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub category: &'static str,
    pub amount: i32,
    pub archived: bool,
}

pub fn summarize(entries: &[Entry]) -> String {
    let mut totals: BTreeMap<&str, (usize, i32)> = BTreeMap::new();

    for entry in entries {
        if entry.archived {
            continue;
        }
        let slot = totals.entry(entry.category).or_insert((0, 0));
        slot.0 += 1;
        slot.1 += entry.amount;
    }

    let mut rows: Vec<_> = totals
        .into_iter()
        .map(|(category, (count, total))| (category, count, total))
        .collect();

    rows.sort_by(|a, b| a.0.cmp(b.0));

    rows.into_iter()
        .map(|(category, count, total)| format!("{}:{}:{}", category, count, total))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::{summarize, Entry};

    #[test]
    fn groups_non_archived_entries_and_sorts_by_total_desc_then_name() {
        let entries = [
            Entry { category: "ops", amount: 5, archived: false },
            Entry { category: "sales", amount: 8, archived: false },
            Entry { category: "ops", amount: 7, archived: false },
            Entry { category: "sales", amount: -3, archived: false },
            Entry { category: "hr", amount: 2, archived: false },
            Entry { category: "ops", amount: 9, archived: true },
        ];

        let got = summarize(&entries);
        let want = "ops:2:12\nsales:2:5\nhr:1:2";
        assert_eq!(got, want);
    }

    #[test]
    fn excludes_categories_whose_final_total_is_zero() {
        let entries = [
            Entry { category: "alpha", amount: 4, archived: false },
            Entry { category: "alpha", amount: -4, archived: false },
            Entry { category: "beta", amount: 3, archived: false },
            Entry { category: "beta", amount: 1, archived: true },
            Entry { category: "gamma", amount: 1, archived: false },
        ];

        let got = summarize(&entries);
        let want = "beta:1:3\ngamma:1:1";
        assert_eq!(got, want);
    }

    #[test]
    fn returns_empty_string_when_no_visible_categories_remain() {
        let entries = [
            Entry { category: "alpha", amount: 2, archived: true },
            Entry { category: "beta", amount: 1, archived: false },
            Entry { category: "beta", amount: -1, archived: false },
        ];

        assert_eq!(summarize(&entries), "");
    }
}
