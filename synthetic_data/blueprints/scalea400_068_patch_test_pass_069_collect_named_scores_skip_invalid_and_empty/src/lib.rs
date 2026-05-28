pub fn collect_named_scores(rows: &[(&str, Option<i32>)]) -> Vec<String> {
    rows.iter()
        .filter_map(|(name, score)| {
            score
                .filter(|s| *s >= 0)
                .map(|s| format!("{}:{}", name.trim(), s))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_named_scores;

    #[test]
    fn skips_missing_negative_and_blank_names() {
        let rows = [
            ("Alice", Some(10)),
            ("", Some(7)),
            ("Bob", None),
            ("Cara", Some(-1)),
            ("  ", Some(3)),
            ("Drew", Some(0)),
        ];

        assert_eq!(collect_named_scores(&rows), vec!["Alice:10", "Drew:0"]);
    }

    #[test]
    fn trims_names_and_preserves_input_order() {
        let rows = [
            ("  Eve ", Some(2)),
            ("Mallory", Some(5)),
            (" Trent", Some(1)),
        ];

        assert_eq!(
            collect_named_scores(&rows),
            vec!["Eve:2", "Mallory:5", "Trent:1"]
        );
    }
}
