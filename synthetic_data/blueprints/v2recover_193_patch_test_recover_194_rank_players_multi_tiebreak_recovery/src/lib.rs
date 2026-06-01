use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub score: u32,
    pub wins: u32,
    pub penalties: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedPlayer {
    pub rank: usize,
    pub name: &'static str,
    pub score: u32,
    pub wins: u32,
    pub penalties: u32,
}

pub fn rank_players(players: &[Player]) -> Vec<RankedPlayer> {
    let mut players = players.to_vec();
    players.sort_by(|a, b| {
        a.score
            .cmp(&b.score)
            .then(a.wins.cmp(&b.wins))
            .then(a.penalties.cmp(&b.penalties))
            .then(a.name.cmp(&b.name))
    });

    players
        .into_iter()
        .enumerate()
        .map(|(i, p)| RankedPlayer {
            rank: i + 1,
            name: p.name,
            score: p.score,
            wins: p.wins,
            penalties: p.penalties,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(rows: &[RankedPlayer]) -> Vec<&'static str> {
        rows.iter().map(|r| r.name).collect()
    }

    fn ranks(rows: &[RankedPlayer]) -> Vec<usize> {
        rows.iter().map(|r| r.rank).collect()
    }

    #[test]
    fn orders_by_score_then_wins_then_penalties_then_name() {
        let players = vec![
            Player { name: "Zed", score: 12, wins: 4, penalties: 2 },
            Player { name: "Amy", score: 12, wins: 4, penalties: 1 },
            Player { name: "Bob", score: 12, wins: 5, penalties: 3 },
            Player { name: "Eli", score: 10, wins: 7, penalties: 0 },
            Player { name: "Cal", score: 12, wins: 4, penalties: 1 },
        ];

        let ranked = rank_players(&players);
        assert_eq!(names(&ranked), vec!["Bob", "Amy", "Cal", "Zed", "Eli"]);
    }

    #[test]
    fn equal_metrics_share_rank_and_later_ranks_skip() {
        let players = vec![
            Player { name: "Amy", score: 20, wins: 6, penalties: 1 },
            Player { name: "Bea", score: 20, wins: 6, penalties: 1 },
            Player { name: "Cara", score: 18, wins: 8, penalties: 0 },
            Player { name: "Dana", score: 16, wins: 9, penalties: 0 },
        ];

        let ranked = rank_players(&players);
        assert_eq!(names(&ranked), vec!["Amy", "Bea", "Cara", "Dana"]);
        assert_eq!(ranks(&ranked), vec![1, 1, 3, 4]);
    }

    #[test]
    fn name_only_breaks_full_ties_without_sharing_rank() {
        let players = vec![
            Player { name: "Mia", score: 15, wins: 5, penalties: 2 },
            Player { name: "Ava", score: 15, wins: 5, penalties: 2 },
            Player { name: "Noah", score: 15, wins: 5, penalties: 2 },
        ];

        let ranked = rank_players(&players);
        assert_eq!(names(&ranked), vec!["Ava", "Mia", "Noah"]);
        assert_eq!(ranks(&ranked), vec![1, 1, 1]);
    }

    #[test]
    fn mixed_ties_and_non_ties_produce_competition_ranks() {
        let players = vec![
            Player { name: "Kai", score: 30, wins: 10, penalties: 0 },
            Player { name: "Lux", score: 30, wins: 10, penalties: 0 },
            Player { name: "Nia", score: 28, wins: 11, penalties: 4 },
            Player { name: "Omar", score: 28, wins: 11, penalties: 4 },
            Player { name: "Pia", score: 28, wins: 10, penalties: 1 },
        ];

        let ranked = rank_players(&players);
        assert_eq!(names(&ranked), vec!["Kai", "Lux", "Nia", "Omar", "Pia"]);
        assert_eq!(ranks(&ranked), vec![1, 1, 3, 3, 5]);
    }
}
