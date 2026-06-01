pub fn collect_visible_scores(entries: &[(&str, Option<i32>, bool)]) -> Vec<String> {
    entries
        .iter()
        .filter(|(_, score, archived)| !archived && score.unwrap_or(0) > 0)
        .map(|(name, score, _)| format!("{}:{}", name, score.unwrap_or(0)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_visible_scores;

    #[test]
    fn keeps_only_active_positive_scores_with_nonempty_names() {
        let items = [
            ("alpha", Some(3), false),
            ("", Some(4), false),
            ("beta", None, false),
            ("gamma", Some(0), false),
            ("delta", Some(8), true),
            ("epsilon", Some(1), false),
        ];

        assert_eq!(
            collect_visible_scores(&items),
            vec!["alpha:3", "epsilon:1"]
        );
    }

    #[test]
    fn trims_names_and_preserves_original_order() {
        let items = [
            ("  kiwi  ", Some(2), false),
            ("mango", Some(5), false),
            ("   ", Some(9), false),
            ("pear", Some(1), true),
            (" plum", Some(4), false),
        ];

        assert_eq!(
            collect_visible_scores(&items),
            vec!["kiwi:2", "mango:5", "plum:4"]
        );
    }
}
