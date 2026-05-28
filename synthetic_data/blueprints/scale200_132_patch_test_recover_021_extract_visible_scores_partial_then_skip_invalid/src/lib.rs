pub fn extract_visible_scores(rows: &[&str]) -> Vec<u32> {
    rows.iter()
        .filter_map(|row| {
            let (name, score) = row.split_once(':')?;
            if name.starts_with('#') || score.is_empty() {
                return None;
            }
            score.parse::<u32>().ok()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::extract_visible_scores;

    #[test]
    fn skips_hidden_and_invalid_rows() {
        let rows = ["alice:10", "#debug:99", "bob:oops", "cara:7"];
        assert_eq!(extract_visible_scores(&rows), vec![10, 7]);
    }

    #[test]
    fn trims_name_and_score_and_skips_blank_names() {
        let rows = ["  dana : 8 ", "   : 4", "erin: 11"];
        assert_eq!(extract_visible_scores(&rows), vec![8, 11]);
    }

    #[test]
    fn hidden_marker_after_name_whitespace_is_still_hidden() {
        let rows = [" #temp:5", "frank:6"];
        assert_eq!(extract_visible_scores(&rows), vec![6]);
    }
}
