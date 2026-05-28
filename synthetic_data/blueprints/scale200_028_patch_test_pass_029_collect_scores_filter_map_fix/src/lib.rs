pub fn collect_scores(entries: &[&str]) -> Vec<u32> {
    entries
        .iter()
        .filter_map(|entry| entry.strip_prefix("score:"))
        .filter_map(|rest| rest.parse::<u32>().ok())
        .filter(|value| *value > 10)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_scores;

    #[test]
    fn keeps_only_score_entries_with_valid_numbers() {
        let input = ["score:7", "note:9", "score:15", "score:abc", "score:10"];
        assert_eq!(collect_scores(&input), vec![7, 15, 10]);
    }

    #[test]
    fn preserves_order_of_parsed_scores() {
        let input = ["ignore", "score:3", "score:22", "score:oops", "score:8"];
        assert_eq!(collect_scores(&input), vec![3, 22, 8]);
    }
}
