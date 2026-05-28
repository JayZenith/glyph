#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub wins: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedPlayer {
    pub rank: usize,
    pub name: String,
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

    fn p(name: &str, score: u32, wins: u32) -> Player {
        Player {
            name: name.to_string(),
            score,
            wins,
        }
    }

    #[test]
    fn sorts_by_score_then_wins_then_name() {
        let ranked = leaderboard(&[
            p("Zed", 12, 3),
            p("Amy", 15, 1),
            p("Bob", 15, 4),
            p("Cara", 15, 4),
            p("Dan", 12, 5),
        ]);

        let names: Vec<_> = ranked.into_iter().map(|r| r.name).collect();
        assert_eq!(names, vec!["Bob", "Cara", "Amy", "Dan", "Zed"]);
    }

    #[test]
    fn ties_share_same_rank_and_next_rank_skips() {
        let ranked = leaderboard(&[
            p("Bob", 20, 5),
            p("Amy", 20, 5),
            p("Cara", 18, 7),
            p("Dan", 18, 6),
        ]);

        let pairs: Vec<_> = ranked.into_iter().map(|r| (r.name, r.rank)).collect();
        assert_eq!(pairs, vec![
            ("Amy".to_string(), 1),
            ("Bob".to_string(), 1),
            ("Cara".to_string(), 3),
            ("Dan".to_string(), 4),
        ]);
    }
}
