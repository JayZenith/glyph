pub fn leaderboard(entries: &[(&str, u32)]) -> Vec<String> {
    let mut players: Vec<(&str, u32)> = entries.to_vec();
    players.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(b.0)));

    let mut out = Vec::new();
    let mut prev_score = None;
    let mut rank = 0usize;

    for (idx, (name, score)) in players.iter().enumerate() {
        if prev_score != Some(*score) {
            rank = idx + 1;
            prev_score = Some(*score);
        }
        out.push(format!("{}. {} ({})", rank, name, score));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::leaderboard;

    #[test]
    fn sorts_by_score_desc_then_name_and_uses_dense_ranks() {
        let got = leaderboard(&[("zoe", 8), ("amy", 10), ("bob", 10), ("clio", 7)]);
        assert_eq!(
            got,
            vec![
                "1. amy (10)",
                "1. bob (10)",
                "3. zoe (8)",
                "4. clio (7)",
            ]
        );
    }

    #[test]
    fn tie_breaks_alphabetically_for_equal_scores() {
        let got = leaderboard(&[("mila", 5), ("ava", 5), ("noah", 5)]);
        assert_eq!(
            got,
            vec!["1. ava (5)", "1. mila (5)", "1. noah (5)"]
        );
    }

    #[test]
    fn empty_input_yields_empty_output() {
        let got = leaderboard(&[]);
        assert!(got.is_empty());
    }
}
