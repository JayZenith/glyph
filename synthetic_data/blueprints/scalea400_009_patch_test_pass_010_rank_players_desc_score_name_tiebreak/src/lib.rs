pub fn leaderboard(entries: &[(&str, u32)]) -> Vec<(usize, String, u32)> {
    let mut items: Vec<(String, u32)> = entries
        .iter()
        .map(|(name, score)| ((*name).to_string(), *score))
        .collect();

    items.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

    let mut out = Vec::new();
    let mut rank = 1usize;

    for (idx, (name, score)) in items.into_iter().enumerate() {
        if idx > 0 {
            rank = idx + 1;
        }
        out.push((rank, name, score));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::leaderboard;

    #[test]
    fn sorts_by_score_desc_then_name_asc() {
        let rows = leaderboard(&[("zoe", 15), ("amy", 20), ("bob", 20), ("ian", 15)]);
        let names: Vec<_> = rows.into_iter().map(|(_, n, _)| n).collect();
        assert_eq!(names, vec!["amy", "bob", "ian", "zoe"]);
    }

    #[test]
    fn ties_share_rank_and_next_rank_skips() {
        let rows = leaderboard(&[("amy", 20), ("bob", 20), ("cyd", 18), ("dan", 17)]);
        let ranks: Vec<_> = rows.into_iter().map(|(r, _, _)| r).collect();
        assert_eq!(ranks, vec![1, 1, 3, 4]);
    }

    #[test]
    fn identical_name_and_score_still_share_rank() {
        let rows = leaderboard(&[("amy", 20), ("amy", 20), ("bob", 19)]);
        let triples: Vec<_> = rows;
        assert_eq!(triples[0].0, 1);
        assert_eq!(triples[1].0, 1);
        assert_eq!(triples[2].0, 3);
    }
}
