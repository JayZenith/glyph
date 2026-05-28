pub fn collect_scores<'a>(events: impl IntoIterator<Item = (&'a str, i32, bool)>) -> Vec<String> {
    events
        .into_iter()
        .filter(|(_, score, active)| *active || *score > 0)
        .map(|(tag, score, _)| (tag.trim(), score))
        .filter(|(tag, _)| !tag.is_empty())
        .map(|(tag, score)| format!("{}:{}", tag, score))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_scores;

    #[test]
    fn keeps_only_active_entries_with_positive_scores() {
        let events = vec![
            ("alpha", 3, true),
            ("beta", 0, true),
            ("gamma", 8, false),
            ("delta", -2, true),
            ("epsilon", 5, true),
        ];

        assert_eq!(
            collect_scores(events),
            vec!["alpha:3", "epsilon:5"]
        );
    }

    #[test]
    fn trims_tags_and_drops_blank_or_inactive_items() {
        let events = vec![
            ("  apple  ", 2, true),
            ("   ", 7, true),
            ("pear", 4, false),
            (" kiwi", 1, true),
        ];

        assert_eq!(collect_scores(events), vec!["apple:2", "kiwi:1"]);
    }
}
