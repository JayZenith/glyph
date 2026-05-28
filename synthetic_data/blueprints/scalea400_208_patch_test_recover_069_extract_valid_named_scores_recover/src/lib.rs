pub fn extract_valid_named_scores(lines: &[&str]) -> Vec<(String, u32)> {
    lines
        .iter()
        .filter_map(|line| {
            let (name, score_text) = line.split_once(':')?;
            let name = name.trim();
            let score = score_text.trim().parse::<u32>().ok()?;

            if name.is_empty() {
                return None;
            }

            Some((name.to_string(), score))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::extract_valid_named_scores;

    #[test]
    fn keeps_only_named_scores_in_range() {
        let input = [
            "alice:10",
            "bob:0",
            "carol:101",
            "dave:42",
            "eve:-1",
            "frank:not-a-number",
            ":55",
        ];

        assert_eq!(
            extract_valid_named_scores(&input),
            vec![
                ("alice".to_string(), 10),
                ("dave".to_string(), 42),
            ]
        );
    }

    #[test]
    fn trims_names_and_rejects_blank_after_trim() {
        let input = ["  amy  : 7", "   : 9", "zoe:100"];

        assert_eq!(
            extract_valid_named_scores(&input),
            vec![
                ("amy".to_string(), 7),
                ("zoe".to_string(), 100),
            ]
        );
    }

    #[test]
    fn rejects_names_containing_whitespace() {
        let input = ["mary jane:50", "otto:8", "x y z:3"];

        assert_eq!(extract_valid_named_scores(&input), vec![("otto".to_string(), 8)]);
    }
}
