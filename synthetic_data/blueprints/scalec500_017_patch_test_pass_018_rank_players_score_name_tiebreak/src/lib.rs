#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub score: u32,
    pub wins: u32,
}

pub fn top_players(players: &[Player], limit: usize) -> Vec<&'static str> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.name.cmp(&b.name))
            .then_with(|| b.wins.cmp(&a.wins))
    });
    items.into_iter().take(limit).map(|p| p.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_players() -> Vec<Player> {
        vec![
            Player { name: "zoe", score: 12, wins: 3 },
            Player { name: "amy", score: 12, wins: 5 },
            Player { name: "bob", score: 9, wins: 9 },
            Player { name: "cora", score: 12, wins: 5 },
            Player { name: "dave", score: 9, wins: 4 },
        ]
    }

    #[test]
    fn ranks_by_score_then_wins_then_name() {
        let got = top_players(&sample_players(), 5);
        assert_eq!(got, vec!["amy", "cora", "zoe", "bob", "dave"]);
    }

    #[test]
    fn limit_is_applied_after_sorting() {
        let got = top_players(&sample_players(), 3);
        assert_eq!(got, vec!["amy", "cora", "zoe"]);
    }

    #[test]
    fn empty_and_oversized_limits_are_handled() {
        assert!(top_players(&sample_players(), 0).is_empty());
        let got = top_players(&sample_players(), 10);
        assert_eq!(got, vec!["amy", "cora", "zoe", "bob", "dave"]);
    }
}
