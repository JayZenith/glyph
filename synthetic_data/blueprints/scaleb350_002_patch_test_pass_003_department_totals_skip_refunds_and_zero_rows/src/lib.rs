use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub department: &'static str,
    pub amount: i32,
}

pub fn summarize(entries: &[Entry]) -> String {
    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();
    for entry in entries {
        *totals.entry(entry.department).or_insert(0) += entry.amount;
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    rows.into_iter()
        .map(|(dept, total)| format!("{dept}:{total}"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_positive_totals_only_sorted_by_total_desc_then_name() {
        let entries = [
            Entry { department: "ops", amount: 5 },
            Entry { department: "sales", amount: 10 },
            Entry { department: "ops", amount: 7 },
            Entry { department: "sales", amount: -3 },
            Entry { department: "hr", amount: 4 },
            Entry { department: "legal", amount: 0 },
            Entry { department: "hr", amount: 8 },
            Entry { department: "alpha", amount: 12 },
            Entry { department: "beta", amount: 12 },
            Entry { department: "ops", amount: -2 },
        ];

        let report = summarize(&entries);
        assert_eq!(report, "alpha:12\nbeta:12\nhr:12\nops:12\nsales:10");
    }

    #[test]
    fn omits_departments_with_only_zero_or_negative_entries() {
        let entries = [
            Entry { department: "ops", amount: 0 },
            Entry { department: "ops", amount: -2 },
            Entry { department: "sales", amount: 3 },
            Entry { department: "sales", amount: 0 },
            Entry { department: "hr", amount: -1 },
        ];

        let report = summarize(&entries);
        assert_eq!(report, "sales:3");
    }
}
