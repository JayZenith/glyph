#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub score: u32,
    pub wins: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedPlayer {
    pub rank: usize,
    pub name: &'static str,
    pub score: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<RankedPlayer> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.name.cmp(&b.name))
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
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orders_by_score_then_wins_then_name() {
        let players = vec![
            Player {
                name: "zoe",
                score: 12,
                wins: 3,
            },
            Player {
                name: "anna",
                score: 15,
                wins: 1,
            },
            Player {
                name: "mike",
                score: 15,
                wins: 4,
            },
            Player {
                name: "bert",
                score: 15,
                wins: 4,
            },
        ];

        let board = leaderboard(&players);
        let names: Vec<_> = board.iter().map(|p| p.name).collect();
        assert_eq!(names, vec!["bert", "mike", "anna", "zoe"]);
        let ranks: Vec<_> = board.iter().map(|p| p.rank).collect();
        assert_eq!(ranks, vec![1, 1, 3, 4]);
    }

    #[test]
    fn shared_rank_only_for_equal_score_and_wins() {
        let players = vec![
            Player {
                name: "amy",
                score: 20,
                wins: 5,
            },
            Player {
                name: "bea",
                score: 20,
                wins: 5,
            },
            Player {
                name: "cam",
                score: 20,
                wins: 4,
            },
            Player {
                name: "dan",
                score: 18,
                wins: 9,
            },
        ];

        let board = leaderboard(&players);
        let pairs: Vec<_> = board.iter().map(|p| (p.name, p.rank)).collect();
        assert_eq!(pairs, vec![("amy", 1), ("bea", 1), ("cam", 3), ("dan", 4)]);
    }
}
