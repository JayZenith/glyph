use std::collections::BTreeMap;

pub fn summarize_hours(entries: &[(&str, i32)]) -> String {
    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();

    for &(dept, hours) in entries {
        *totals.entry(dept).or_insert(0) += hours;
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    rows.into_iter()
        .map(|(dept, total)| format!("{}:{}", dept, total))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::summarize_hours;

    #[test]
    fn groups_and_sorts_by_total_desc_then_name() {
        let entries = [
            ("ops", 3),
            ("sales", 5),
            ("ops", 4),
            ("hr", 5),
            ("sales", 1),
        ];

        assert_eq!(summarize_hours(&entries), "ops:7\nsales:6\nhr:5");
    }

    #[test]
    fn ignores_invalid_entries_before_aggregating() {
        let entries = [
            ("", 4),
            ("support", 0),
            ("support", -2),
            ("sales", 2),
            ("sales", 3),
            ("hr", 5),
        ];

        assert_eq!(summarize_hours(&entries), "hr:5\nsales:5");
    }
}
