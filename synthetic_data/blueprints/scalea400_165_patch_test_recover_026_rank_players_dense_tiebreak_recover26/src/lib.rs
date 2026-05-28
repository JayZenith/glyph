#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player<'a> {
    pub name: &'a str,
    pub score: u32,
    pub wins: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedPlayer<'a> {
    pub rank: usize,
    pub name: &'a str,
    pub score: u32,
    pub wins: u32,
}

pub fn top_ranked<'a>(players: &[Player<'a>], top_k: usize) -> Vec<RankedPlayer<'a>> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.name.cmp(b.name))
            .then_with(|| b.wins.cmp(&a.wins))
    });

    items
        .into_iter()
        .enumerate()
        .map(|(idx, p)| RankedPlayer {
            rank: idx + 1,
            name: p.name,
            score: p.score,
            wins: p.wins,
        })
        .take(top_k)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orders_by_score_then_wins_then_name() {
        let players = vec![
            Player { name: "zoe", score: 12, wins: 3 },
            Player { name: "amy", score: 15, wins: 1 },
            Player { name: "bob", score: 15, wins: 4 },
            Player { name: "ava", score: 15, wins: 4 },
            Player { name: "mia", score: 12, wins: 8 },
        ];

        let ranked = top_ranked(&players, 5);
        let names: Vec<_> = ranked.iter().map(|p| p.name).collect();
        assert_eq!(names, vec!["ava", "bob", "amy", "mia", "zoe"]);
    }

    #[test]
    fn uses_dense_ranks_for_ties() {
        let players = vec![
            Player { name: "amy", score: 20, wins: 5 },
            Player { name: "bea", score: 20, wins: 5 },
            Player { name: "cam", score: 18, wins: 9 },
            Player { name: "dan", score: 18, wins: 9 },
            Player { name: "eli", score: 17, wins: 1 },
        ];

        let ranked = top_ranked(&players, 5);
        let pairs: Vec<_> = ranked.iter().map(|p| (p.name, p.rank)).collect();
        assert_eq!(pairs, vec![
            ("amy", 1),
            ("bea", 1),
            ("cam", 2),
            ("dan", 2),
            ("eli", 3),
        ]);
    }

    #[test]
    fn top_k_is_applied_after_ranking() {
        let players = vec![
            Player { name: "ivy", score: 30, wins: 1 },
            Player { name: "ian", score: 30, wins: 1 },
            Player { name: "gus", score: 28, wins: 7 },
            Player { name: "hal", score: 27, wins: 9 },
        ];

        let ranked = top_ranked(&players, 3);
        assert_eq!(ranked, vec![
            RankedPlayer { rank: 1, name: "ian", score: 30, wins: 1 },
            RankedPlayer { rank: 1, name: "ivy", score: 30, wins: 1 },
            RankedPlayer { rank: 2, name: "gus", score: 28, wins: 7 },
        ]);
    }
}
