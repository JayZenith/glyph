#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub score: u32,
    pub penalties: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedPlayer {
    pub rank: usize,
    pub name: &'static str,
    pub score: u32,
    pub penalties: u32,
}

pub fn rank_players(players: &[Player]) -> Vec<RankedPlayer> {
    let mut ordered = players.to_vec();
    ordered.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| b.penalties.cmp(&a.penalties))
            .then_with(|| b.name.cmp(a.name))
    });

    ordered
        .into_iter()
        .enumerate()
        .map(|(idx, p)| RankedPlayer {
            rank: idx + 1,
            name: p.name,
            score: p.score,
            penalties: p.penalties,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_score_penalties_then_name() {
        let players = vec![
            Player { name: "Zoe", score: 15, penalties: 2 },
            Player { name: "Amy", score: 15, penalties: 2 },
            Player { name: "Bob", score: 15, penalties: 1 },
            Player { name: "Ian", score: 12, penalties: 0 },
        ];

        let names: Vec<_> = rank_players(&players).into_iter().map(|p| p.name).collect();
        assert_eq!(names, vec!["Bob", "Amy", "Zoe", "Ian"]);
    }

    #[test]
    fn equal_score_and_penalty_share_rank_with_gap() {
        let players = vec![
            Player { name: "Bob", score: 20, penalties: 1 },
            Player { name: "Amy", score: 20, penalties: 1 },
            Player { name: "Cid", score: 19, penalties: 0 },
            Player { name: "Dan", score: 18, penalties: 0 },
        ];

        let ranked = rank_players(&players);
        let summary: Vec<_> = ranked.into_iter().map(|p| (p.rank, p.name)).collect();
        assert_eq!(summary, vec![(1, "Amy"), (1, "Bob"), (3, "Cid"), (4, "Dan")]);
    }

    #[test]
    fn ties_depend_only_on_score_and_penalties_not_name() {
        let players = vec![
            Player { name: "Kai", score: 9, penalties: 3 },
            Player { name: "Ava", score: 9, penalties: 3 },
            Player { name: "Mia", score: 9, penalties: 4 },
        ];

        let ranked = rank_players(&players);
        let summary: Vec<_> = ranked.into_iter().map(|p| (p.rank, p.name, p.penalties)).collect();
        assert_eq!(summary, vec![(1, "Ava", 3), (1, "Kai", 3), (3, "Mia", 4)]);
    }
}
