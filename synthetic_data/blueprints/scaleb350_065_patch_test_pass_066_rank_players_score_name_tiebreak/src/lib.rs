use std::cmp::Ordering;

pub fn rank_players(players: &[(&str, u32)]) -> Vec<String> {
    let mut items: Vec<(&str, u32)> = players.to_vec();
    items.sort_by(|a, b| match b.1.cmp(&a.1) {
        Ordering::Equal => b.0.cmp(a.0),
        other => other,
    });
    items.into_iter().map(|(name, _)| name.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::rank_players;

    #[test]
    fn sorts_by_score_descending() {
        let players = [("zoe", 7), ("amy", 12), ("max", 9)];
        assert_eq!(rank_players(&players), vec!["amy", "max", "zoe"]);
    }

    #[test]
    fn breaks_ties_by_name_ascending() {
        let players = [("zoe", 10), ("amy", 10), ("bob", 10)];
        assert_eq!(rank_players(&players), vec!["amy", "bob", "zoe"]);
    }

    #[test]
    fn mixes_scores_and_ties() {
        let players = [("mia", 5), ("ava", 8), ("ivy", 8), ("noa", 3)];
        assert_eq!(rank_players(&players), vec!["ava", "ivy", "mia", "noa"]);
    }
}
