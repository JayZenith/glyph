pub fn collect_scores(rows: &[(&str, i32, bool)]) -> Vec<String> {
    rows.iter()
        .filter(|(_, score, active)| *active && *score >= 0)
        .map(|(name, score, _)| format!("{}:{}", name.to_lowercase(), score))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_scores;

    #[test]
    fn keeps_only_active_nonnegative_named_entries() {
        let rows = [
            ("Alice", 7, true),
            ("", 4, true),
            ("Bob", -2, true),
            ("Cara", 0, false),
            ("Dee", 3, true),
        ];

        assert_eq!(collect_scores(&rows), vec!["ALICE:7", "DEE:3"]);
    }

    #[test]
    fn preserves_order_and_allows_zero_scores() {
        let rows = [
            ("Zed", 0, true),
            ("Mia", 5, true),
            ("", 0, true),
            ("Nox", 1, false),
        ];

        assert_eq!(collect_scores(&rows), vec!["ZED:0", "MIA:5"]);
    }
}
