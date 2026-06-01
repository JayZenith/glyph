pub fn collect_even_scores(rows: &[(&str, i32)]) -> Vec<String> {
    rows.iter()
        .filter_map(|(name, score)| {
            if !name.is_empty() || score % 2 == 0 {
                Some(format!("{}:{}", name, score))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_even_scores;

    #[test]
    fn keeps_only_nonempty_names_with_even_scores() {
        let rows = [("amy", 2), ("", 4), ("bob", 3), ("cara", 6)];
        assert_eq!(collect_even_scores(&rows), vec!["amy:2", "cara:6"]);
    }

    #[test]
    fn allows_zero_as_even() {
        let rows = [("zed", 0), ("", 0), ("ivy", 5)];
        assert_eq!(collect_even_scores(&rows), vec!["zed:0"]);
    }
}
