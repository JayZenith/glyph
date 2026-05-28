use std::collections::BTreeMap;

pub fn summarize_by_team(records: &[(&str, i32)]) -> String {
    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();

    for (team, amount) in records {
        *totals.entry(*team).or_insert(0) += *amount;
    }

    let mut rows: Vec<(&str, i32)> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    rows.into_iter()
        .map(|(team, total)| format!("{}:{}", team, total))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::summarize_by_team;

    #[test]
    fn sorts_by_total_desc_then_name_and_skips_zero_totals() {
        let records = [
            ("ops", 4),
            ("sales", 2),
            ("ops", -1),
            ("hr", 0),
            ("sales", 1),
            ("dev", 3),
            ("qa", -2),
            ("qa", 2),
        ];

        let report = summarize_by_team(&records);
        assert_eq!(report, "dev:3\nops:3\nsales:3");
    }

    #[test]
    fn keeps_negative_totals_after_positive_totals() {
        let records = [
            ("blue", -1),
            ("red", 5),
            ("green", -3),
            ("blue", -1),
            ("red", -2),
        ];

        let report = summarize_by_team(&records);
        assert_eq!(report, "red:3\nblue:-2\ngreen:-3");
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let report = summarize_by_team(&[]);
        assert!(report.is_empty());
    }
}
