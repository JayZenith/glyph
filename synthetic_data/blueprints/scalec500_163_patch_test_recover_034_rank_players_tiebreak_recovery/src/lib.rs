#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub score: u32,
    pub wins: u32,
    pub penalty: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedPlayer {
    pub rank: usize,
    pub name: &'static str,
    pub score: u32,
    pub wins: u32,
    pub penalty: u32,
}

pub fn rank_players(players: &[Player]) -> Vec<RankedPlayer> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.penalty.cmp(&b.penalty))
            .then_with(|| a.name.cmp(&b.name))
    });

    let mut out = Vec::with_capacity(items.len());
    for (idx, p) in items.into_iter().enumerate() {
        out.push(RankedPlayer {
            rank: idx + 1,
            name: p.name,
            score: p.score,
            wins: p.wins,
            penalty: p.penalty,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orders_by_score_then_wins_then_penalty_then_name() {
        let players = vec![
            Player { name: "Zoe", score: 20, wins: 3, penalty: 7 },
            Player { name: "Amy", score: 20, wins: 5, penalty: 10 },
            Player { name: "Ben", score: 20, wins: 5, penalty: 8 },
            Player { name: "Ada", score: 20, wins: 5, penalty: 8 },
            Player { name: "Ian", score: 18, wins: 9, penalty: 1 },
        ];

        let ranked = rank_players(&players);
        let names: Vec<_> = ranked.iter().map(|p| p.name).collect();
        assert_eq!(names, vec!["Ada", "Ben", "Amy", "Zoe", "Ian"]);
    }

    #[test]
    fn ties_share_rank_when_score_wins_and_penalty_match() {
        let players = vec![
            Player { name: "Cara", score: 30, wins: 6, penalty: 4 },
            Player { name: "Bea", score: 30, wins: 6, penalty: 4 },
            Player { name: "Ava", score: 30, wins: 6, penalty: 5 },
            Player { name: "Dex", score: 28, wins: 8, penalty: 1 },
        ];

        let ranked = rank_players(&players);
        let pairs: Vec<_> = ranked.iter().map(|p| (p.rank, p.name)).collect();
        assert_eq!(pairs, vec![(1, "Bea"), (1, "Cara"), (3, "Ava"), (4, "Dex")]);
    }

    #[test]
    fn empty_input_returns_empty() {
        let ranked = rank_players(&[]);
        assert!(ranked.is_empty());
    }
}
