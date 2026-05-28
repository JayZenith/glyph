pub fn leaderboard(entries: &[(&str, u32, u32)]) -> Vec<String> {
    let mut rows: Vec<(&str, u32, u32)> = entries.to_vec();
    rows.sort_by(|a, b| {
        b.1.cmp(&a.1)
            .then(a.2.cmp(&b.2))
            .then(a.0.cmp(&b.0))
    });

    rows.into_iter()
        .map(|(name, score, penalty)| format!("{}:{}:{}", name, score, penalty))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::leaderboard;

    #[test]
    fn sorts_by_score_then_penalty_then_name() {
        let rows = [
            ("zoe", 15, 3),
            ("amy", 20, 5),
            ("bob", 20, 2),
            ("cara", 20, 2),
            ("dan", 15, 1),
        ];

        let ranked = leaderboard(&rows);
        assert_eq!(
            ranked,
            vec![
                "bob:20:2",
                "cara:20:2",
                "amy:20:5",
                "dan:15:1",
                "zoe:15:3",
            ]
        );
    }

    #[test]
    fn keeps_duplicate_scores_but_orders_names_for_exact_ties() {
        let rows = [
            ("mila", 8, 4),
            ("ava", 8, 4),
            ("noah", 9, 7),
            ("liam", 9, 7),
        ];

        let ranked = leaderboard(&rows);
        assert_eq!(ranked, vec!["liam:9:7", "noah:9:7", "ava:8:4", "mila:8:4"]);
    }
}
