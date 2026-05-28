pub fn collect_visible_scores(items: &[(&str, Option<i32>, bool)]) -> Vec<String> {
    items
        .iter()
        .filter_map(|(name, score, hidden)| {
            if *hidden {
                return None;
            }

            score.map(|s| format!("{}:{}", name, s))
        })
        .filter(|entry| {
            entry.split_once(':')
                .map(|(_, n)| n.parse::<i32>().unwrap_or(0) >= 0)
                .unwrap_or(false)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_visible_scores;

    #[test]
    fn keeps_only_public_named_nonnegative_scores_in_input_order() {
        let items = [
            ("alpha", Some(3), false),
            ("", Some(7), false),
            ("beta", None, false),
            ("gamma", Some(-2), false),
            ("delta", Some(0), true),
            ("epsilon", Some(1), false),
        ];

        assert_eq!(
            collect_visible_scores(&items),
            vec!["alpha:3", "epsilon:1"]
        );
    }

    #[test]
    fn trims_names_and_preserves_zero_scores() {
        let items = [
            ("  ivy  ", Some(0), false),
            (" oak", Some(5), false),
            ("   ", Some(9), false),
        ];

        assert_eq!(
            collect_visible_scores(&items),
            vec!["ivy:0", "oak:5"]
        );
    }
}
