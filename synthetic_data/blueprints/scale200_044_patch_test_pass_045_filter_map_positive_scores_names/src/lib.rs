pub fn passing_names(entries: &[(&str, Option<i32>)]) -> Vec<String> {
    entries
        .iter()
        .filter_map(|(name, score)| match score {
            Some(value) if *value >= 0 => Some((*name).to_string()),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::passing_names;

    #[test]
    fn keeps_only_positive_scores_with_names() {
        let entries = [
            ("amy", Some(3)),
            ("bob", Some(0)),
            ("cara", None),
            ("dan", Some(-2)),
            ("eve", Some(7)),
        ];

        assert_eq!(passing_names(&entries), vec!["amy", "eve"]);
    }

    #[test]
    fn empty_when_no_positive_scores() {
        let entries = [("amy", Some(0)), ("bob", Some(-1)), ("cara", None)];
        let got: Vec<String> = Vec::new();
        assert_eq!(passing_names(&entries), got);
    }
}
