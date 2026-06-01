use std::collections::BTreeMap;

pub fn summarize_hours(entries: &[(&str, u32)]) -> Vec<String> {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();
    for (team, hours) in entries {
        *totals.entry(*team).or_insert(0) += *hours;
    }

    totals
        .into_iter()
        .map(|(team, total)| format!("{}:{}h", team, total))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::summarize_hours;

    #[test]
    fn aggregates_duplicate_teams() {
        let entries = [
            ("ops", 3),
            ("sales", 5),
            ("ops", 4),
            ("sales", 1),
            ("hr", 2),
        ];
        assert_eq!(
            summarize_hours(&entries),
            vec!["hr:2h", "ops:7h", "sales:6h"]
        );
    }

    #[test]
    fn empty_input_returns_empty_report() {
        let entries: [(&str, u32); 0] = [];
        assert!(summarize_hours(&entries).is_empty());
    }

    #[test]
    fn preserves_sorted_team_order_in_report() {
        let entries = [("zeta", 1), ("alpha", 2), ("gamma", 3)];
        assert_eq!(
            summarize_hours(&entries),
            vec!["alpha:2h", "gamma:3h", "zeta:1h"]
        );
    }
}
