pub fn collect_visible_scores(rows: &[&str]) -> Vec<String> {
    rows.iter()
        .filter_map(|row| row.split_once(':'))
        .filter_map(|(name, score)| {
            let name = name.trim();
            let score = score.trim();
            score.parse::<u32>().ok().map(|n| format!("{}={}", name, n))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_visible_scores;

    #[test]
    fn keeps_only_visible_valid_nonzero_scores() {
        let rows = [
            "alice:10",
            "bob:hidden:4",
            "cara:0",
            "drew:7",
            "  ella  :  5  ",
            "frank:hidden:0",
            "gina:bad",
        ];

        assert_eq!(
            collect_visible_scores(&rows),
            vec!["alice=10", "drew=7", "ella=5"]
        );
    }

    #[test]
    fn ignores_blank_names_and_blank_scores() {
        let rows = [
            " :4",
            "ivy:",
            "jane: 3",
            "kate:hidden:9",
            "leo:1",
        ];

        assert_eq!(collect_visible_scores(&rows), vec!["jane=3", "leo=1"]);
    }
}
