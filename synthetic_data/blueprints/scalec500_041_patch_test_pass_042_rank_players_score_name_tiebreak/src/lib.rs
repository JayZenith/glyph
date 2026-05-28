pub fn leaderboard(entries: &[(&str, u32)]) -> Vec<String> {
    let mut rows: Vec<(&str, u32)> = entries.iter().map(|(name, score)| (*name, *score)).collect();
    rows.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(b.0)));

    let mut out = Vec::new();
    let mut last_score = None;
    let mut rank = 0usize;

    for (idx, (name, score)) in rows.iter().enumerate() {
        if last_score != Some(*score) {
            rank = idx + 1;
            last_score = Some(*score);
        }
        out.push(format!("{}:{}:{}", rank, name, score));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::leaderboard;

    #[test]
    fn orders_by_score_desc_then_name_asc_with_competition_ranks() {
        let got = leaderboard(&[
            ("zoe", 10),
            ("amy", 30),
            ("bob", 30),
            ("eve", 20),
            ("dan", 20),
        ]);

        assert_eq!(
            got,
            vec![
                "1:amy:30",
                "1:bob:30",
                "3:dan:20",
                "3:eve:20",
                "5:zoe:10",
            ]
        );
    }

    #[test]
    fn equal_scores_are_sorted_alphabetically() {
        let got = leaderboard(&[("mila", 7), ("ava", 7), ("noah", 7)]);
        assert_eq!(got, vec!["1:ava:7", "1:mila:7", "1:noah:7"]);
    }
}
