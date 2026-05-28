#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub score: u32,
    pub wins: u32,
    pub losses: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedPlayer {
    pub rank: usize,
    pub name: &'static str,
    pub score: u32,
    pub wins: u32,
    pub losses: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<RankedPlayer> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.name.cmp(&b.name))
    });

    items
        .into_iter()
        .enumerate()
        .map(|(idx, p)| RankedPlayer {
            rank: idx + 1,
            name: p.name,
            score: p.score,
            wins: p.wins,
            losses: p.losses,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_all_tiebreakers() {
        let ranked = leaderboard(&[
            Player { name: "Zed", score: 20, wins: 4, losses: 0 },
            Player { name: "Ava", score: 20, wins: 5, losses: 3 },
            Player { name: "Mia", score: 20, wins: 5, losses: 1 },
            Player { name: "Bea", score: 20, wins: 5, losses: 1 },
            Player { name: "Ian", score: 18, wins: 9, losses: 0 },
        ]);

        let names: Vec<_> = ranked.iter().map(|p| p.name).collect();
        assert_eq!(names, vec!["Bea", "Mia", "Ava", "Zed", "Ian"]);
    }

    #[test]
    fn uses_competition_ranking_for_exact_ties() {
        let ranked = leaderboard(&[
            Player { name: "Ada", score: 30, wins: 8, losses: 2 },
            Player { name: "Ben", score: 30, wins: 8, losses: 2 },
            Player { name: "Cam", score: 29, wins: 9, losses: 0 },
            Player { name: "Dot", score: 28, wins: 7, losses: 1 },
        ]);

        let pairs: Vec<_> = ranked.iter().map(|p| (p.name, p.rank)).collect();
        assert_eq!(pairs, vec![("Ada", 1), ("Ben", 1), ("Cam", 3), ("Dot", 4)]);
    }
}
