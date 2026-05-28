pub fn passing_names(entries: &[(&str, Option<u32>)], min_score: u32) -> Vec<String> {
    entries
        .iter()
        .filter_map(|(name, score)| match score {
            Some(value) if *value <= min_score => Some((*name).to_string()),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::passing_names;

    #[test]
    fn includes_only_scores_at_or_above_threshold() {
        let entries = [
            ("Ana", Some(72)),
            ("Ben", None),
            ("Cara", Some(80)),
            ("Dan", Some(67)),
        ];

        assert_eq!(passing_names(&entries, 70), vec!["Ana", "Cara"]);
    }

    #[test]
    fn empty_when_nobody_has_valid_passing_score() {
        let entries = [("A", None), ("B", Some(49)), ("C", Some(50))];
        assert_eq!(passing_names(&entries, 60), Vec::<String>::new());
    }
}
