pub fn collect_scores(lines: &[&str]) -> Vec<(String, u32)> {
    lines
        .iter()
        .filter_map(|line| line.split_once(':'))
        .map(|(name, raw)| (name.trim().to_string(), raw.trim().parse::<i32>().unwrap_or(0)))
        .filter(|(_, score)| *score >= 0)
        .map(|(name, score)| (name, score as u32))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_scores;

    #[test]
    fn keeps_only_nonempty_names_with_positive_scores() {
        let input = [
            "alice: 3",
            "bob: 0",
            " : 9",
            "carol: -2",
            "dave: nope",
            "erin: 7",
        ];

        assert_eq!(collect_scores(&input), vec![
            ("alice".to_string(), 3),
            ("erin".to_string(), 7),
        ]);
    }

    #[test]
    fn trims_fields_and_ignores_extra_colons_after_number_parse_failure() {
        let input = [
            " frank : 12 ",
            "grace:4:extra",
            "heidi: 1",
            "ivan:",
        ];

        assert_eq!(collect_scores(&input), vec![
            ("frank".to_string(), 12),
            ("heidi".to_string(), 1),
        ]);
    }
}
